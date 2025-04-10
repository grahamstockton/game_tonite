use leptos::{logging::log, prelude::*};
use reactive_stores::Store;

use crate::{
    app::{GlobalState, GlobalStateStoreFields},
    component::model::User,
    obf_util::UrlParamsStoreFields as _,
};

#[component]
pub fn JoinLeaveSessionButton(session_id: i64) -> impl IntoView {
    // get state
    let state = expect_context::<Store<GlobalState>>();
    let user_id = state.url_params().user_id().get_untracked();

    // TODO : get user id from state

    // get if user is participating
    let calendar_events = state.calendar_events();
    let is_user_participating = move || {
        let user_id = state.url_params().user_id().get_untracked();
        calendar_events
            .get()
            .iter()
            .find(|s| {
                s.session_id == session_id && s.participants.iter().any(|p| p.get_name() == user_id)
            })
            .is_some()
    };

    // handle RemoveUser ActionForm
    let remove_user = ServerAction::<RemoveUser>::new();
    let server_res = remove_user.value();
    Effect::new(move || match server_res() {
        Some(Ok(())) => state.calendar_events().update(|v| {
            let user_id = state.url_params().user_id().get_untracked();
            if let Some(session) = v.iter_mut().find(|s| s.session_id == session_id) {
                if let Some(idx) = session
                    .participants
                    .iter()
                    .position(|p| p.get_name() == user_id.clone())
                {
                    session.participants.remove(idx);
                }
            }
        }),
        Some(Err(e)) => {
            log!("{:?}", e);
        }
        None => {}
    });

    // handle AddUser ActionForm
    let add_user = ServerAction::<AddUser>::new();
    let server_res = add_user.value();
    Effect::new(move || match server_res() {
        Some(Ok(())) => state.calendar_events().update(|v| {
            let user_id = state.url_params().user_id().get_untracked();
            if let Some(session) = v.iter_mut().find(|s| s.session_id == session_id) {
                if !session
                    .participants
                    .iter()
                    .any(|p| p.get_name() == user_id.clone())
                {
                    session.participants.push(User {
                        name: user_id.clone(),
                        picture: "placeholder".to_string(),
                    })
                }
            }
        }),
        Some(Err(e)) => {
            log!("{:?}", e);
        }
        None => {}
    });

    view! {
        {
            if is_user_participating() {
                view! {
                    <ActionForm action=remove_user>
                        <input type="text" class="hidden invisible" name="session_id" value={session_id}/>
                        <input type="text" class="hidden invisible" name="user_id" value={user_id}/>
                        <button class="btn btn-round">{"-"}</button>
                    </ActionForm>
                }.into_any()
            } else {
                view! {
                    <ActionForm action=add_user>
                        <input type="text" class="hidden invisible" name="session_id" value={session_id}/>
                        <input type="text" class="hidden invisible" name="user_id" value={user_id}/>
                        <button class="btn btn-round">{"+"}</button>
                    </ActionForm>
                }.into_any()
            }
        }
    }
}

#[server]
pub async fn add_user(user_id: String, session_id: String) -> Result<(), ServerFnError> {
    use crate::dao::sqlite_util::SqliteClient;
    use sqlx::{Pool, Sqlite};

    let pool = use_context::<Pool<Sqlite>>().expect("pool not found");
    let client = SqliteClient::from_pool(pool).await;

    match client
        .create_session_user(&user_id, session_id.parse::<i64>().unwrap(), "placeholder")
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => Err(ServerFnError::new(e)),
    }
}

#[server]
pub async fn remove_user(user_id: String, session_id: String) -> Result<(), ServerFnError> {
    use crate::dao::sqlite_util::SqliteClient;
    use sqlx::{Pool, Sqlite};

    let pool = use_context::<Pool<Sqlite>>().expect("pool not found");
    let client = SqliteClient::from_pool(pool).await;

    match client
        .delete_session_user(session_id.parse::<i64>().unwrap(), &user_id)
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => Err(ServerFnError::new(e)),
    }
}
