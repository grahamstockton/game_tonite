use leptos::{html::Dialog, logging::log, prelude::*};
use reactive_stores::Store;

use crate::{
    app::{GlobalState, GlobalStateStoreFields as _},
    obf_util::UrlParamsStoreFields,
};

#[component]
pub fn DeleteEventModal(session_id: i64, owner_id: String) -> impl IntoView {
    let state = expect_context::<Store<GlobalState>>();
    let calendar_events = state.calendar_events();
    let user_id = state.url_params().user_id().get_untracked();
    let modal_name = format!("modal_{}", session_id);

    // noderef and error signal (window)
    let (error_status, set_error_status) = signal(false);

    // sometimes modals replace each other upon deletion
    let e = NodeRef::<Dialog>::new();

    // handle ActionForm
    let delete_event = ServerAction::<DeleteEvent>::new();
    let server_res = delete_event.value();
    Effect::new(move || match server_res() {
        Some(Ok(())) => {
            calendar_events.update(|v| {
                v.retain(|i| i.session_id != session_id);
            });
            e.get().unwrap().close();
        }
        Some(Err(e)) => {
            set_error_status(true);
            log!("{:?}", e);
        }
        None => {}
    });

    // only show if user owns event
    if user_id == owner_id {
        view! {
            <button type="button" onclick={format!("{}.showModal()", modal_name)} class="btn btn-sm btn-circle btn-ghost">{"✕"}</button>
            <dialog node_ref=e id={modal_name} class="modal">
            <div class="modal-box">
                <div class="flex">
                    <h3 class="text-lg flex-1 font-bold">Are you sure?</h3>
                    <form class="dialog flex-0">
                        <button type="button" onclick={format!("{}.close()", modal_name)} class="btn btn-sm btn-circle btn-ghost">{"✕"}</button>
                    </form>
                </div>
                <p class="py-4">Deleting events is permanent</p>
                {
                    move || if error_status() {view!{
                        <div role="alert" class="alert alert-error">
                            <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6 shrink-0 stroke-current" fill="none" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" />
                            </svg>
                            <span>Error! Please Try Again</span>
                        </div>
                    }.into_any()} 
                    else { view! {}.into_any() }
                }
                <div class="modal-action">
                    <ActionForm action=delete_event>
                        <input type="text" class="hidden invisible" name="session_id" value={session_id}/>
                        <button class="btn btn-error">Delete</button>
                    </ActionForm>
                </div>
            </div>
            </dialog>
        }.into_any()
    } else {
        view! {}.into_any()
    }
}

#[server]
pub async fn delete_event(session_id: i64) -> Result<(), ServerFnError> {
    use crate::dao::sqlite_util::SqliteClient;
    use sqlx::{Pool, Sqlite};

    let pool = use_context::<Pool<Sqlite>>().expect("pool not found");
    let client = SqliteClient::from_pool(pool).await;

    match client.delete_session(session_id).await {
        Ok(()) => Ok(()),
        Err(e) => Err(ServerFnError::new(e)),
    }
}
