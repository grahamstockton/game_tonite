use chrono::{DateTime, FixedOffset};
use leptos::{html::Dialog, logging::log, prelude::*};
use reactive_stores::Store;

use crate::{
    app::{GlobalState, GlobalStateStoreFields},
    component::{
        model::{GamingSession, User},
        time_util::{convert_simple_time, create_baseline, get_local_time},
    },
    obf_util::UrlParamsStoreFields,
};

/**
 * Modal form to create a new event
 */
#[component]
pub fn NewEventModal() -> impl IntoView {
    let state = expect_context::<Store<GlobalState>>();
    let server_id = state.url_params().server_id().get_untracked();
    let user_id = state.url_params().user_id().get_untracked();
    let calendar_events = state.calendar_events();

    let e = NodeRef::<Dialog>::new();
    let (error_status, set_error_status) = signal(false);

    // current time for timestamp
    let (local_time, set_local_time) =
        signal::<DateTime<FixedOffset>>(DateTime::from_timestamp(0, 0).unwrap().fixed_offset());
    Effect::new(move || {
        // set time locally
        let t = get_local_time();
        set_local_time(t);
    });

    // handle ActionForm
    let create_event = ServerAction::<CreateEvent>::new();
    let server_res = create_event.value();
    Effect::new(move || match server_res() {
        Some(Ok(s)) => {
            calendar_events.update(|v| v.push(s));
            e.get().unwrap().close();
            set_error_status(false);
        }
        Some(Err(e)) => {
            log!("{:?}", e);
            set_error_status(true);
        }
        None => {}
    });

    view! {
        <div class="absolute bottom-5 right-5">
            <button class="btn btn-xl btn-circle btn-secondary" onclick="my_modal_1.showModal()">
                <svg width="48" height="48" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor" className="size-6">
                    <path strokeLinecap="round" strokeLinejoin="round" d="M12 4.5v15m7.5-7.5h-15" />
                </svg>
            </button>
        </div>
        <dialog node_ref=e id="my_modal_1" class="modal modal">
            <div class="modal-box w-80">
                <div class="modal-action mt-0 flex-col">
                    <div class="flex">
                        <h3 class="text-lg flex-1 font-bold">Create Event</h3>
                        <form class="dialog flex-0">
                            <button type="button" onclick="my_modal_1.close()" class="btn btn-sm btn-circle btn-ghost">{"✕"}</button>
                        </form>
                    </div>
                    <ActionForm action=create_event>
                        // hidden vars for action form -- will change if there is a better fix
                        <input type="text" class="hidden invisible" name="server_id" value={server_id}/>
                        <input type="text" class="hidden invisible" name="user_id" value={user_id.clone()}/>
                        <input type="text" class="hidden invisible" name="owner" value={user_id}/>
                        <input type="text" class="hidden invisible" name="picture" value={"placeholder"}/>
                        <input type="text" class="hidden invisible" name="baseline_time" value={move || local_time().to_rfc3339()} />
                        <input type="text" class="hidden invisible" name="offset" value={state.offset().get_untracked()} />
                        <fieldset class="fieldset w-full bg-base-200 border border-base-300 p-4 rounded-box">
                            {
                                move || if error_status() {
                                    view! {
                                        <div role="alert" class="alert alert-error">
                                            <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6 shrink-0 stroke-current" fill="none" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" />
                                            </svg>
                                            <span>Error! Please Try Again</span>
                                        </div>
                                    }.into_any()
                                } else {
                                    view! {}.into_any()
                                }
                            }
                            <legend class="fieldset-legend">Event</legend>

                            <label class="fieldset-label">Title</label>
                            <input type="text" class="input" placeholder="Title" name="title" maxlength="30" required />

                            <label class="fieldset-label">Start Time</label>
                            <input type="time" class="input" name="start" required />

                            <label class="fieldset-label">End Time</label>
                            <input type="time" class="input" name="end" required />

                            <label class="fieldset-label">Game (optional)</label>
                            <input type="text" class="input" name="game" maxlength="30" />

                            <button type="submit" class="btn btn-neutral mt-4">Create</button>
                        </fieldset>
                    </ActionForm>
                </div>
            </div>
        </dialog>
    }
}

// TODO: How do I pass in additional things to this fn that aren't just from the forms
#[server]
pub async fn create_event(
    title: String,
    start: String,
    end: String,
    server_id: String,
    user_id: String,
    owner: String,
    picture: String,
    baseline_time: String,
    offset: String,
    game: String,
) -> Result<GamingSession, ServerFnError> {
    use crate::dao::sqlite_util::SqliteClient;
    // TODO: test this, then use extractors to share an sqlite client across instances
    let client = SqliteClient::new("sqlite://sessions.db").await;
    let offset_usize: usize = offset.parse().unwrap();
    let game_opt = if game.trim().is_empty() {
        None
    } else {
        Some(game)
    };

    let baseline = DateTime::parse_from_rfc3339(&baseline_time)?;

    let adjusted_baseline = create_baseline(baseline, offset_usize)
        .map_err(|e| ServerFnError::new("failed to adjust baseline"))?;
    let start_datetime = convert_simple_time(start, adjusted_baseline, offset_usize);
    let end_datetime = convert_simple_time(end, adjusted_baseline, offset_usize);

    let session_record = client
        .create_session(
            &server_id,
            &title,
            &start_datetime.to_rfc3339(),
            &end_datetime.to_rfc3339(),
            &owner,
            game_opt.clone(),
        )
        .await
        .map_err(|e| ServerFnError::new(format!("failed to create session: {}", e)))?;
    let session_id = session_record.session_id.unwrap();
    let user_result = client
        .create_session_user(&user_id, session_id, &picture)
        .await;

    match user_result {
        Ok(_) => {
            let user = User {
                name: user_id,
                picture: picture,
            };
            Ok(GamingSession {
                server_id: server_id,
                session_id: session_id,
                title: title,
                start_time: start_datetime.to_utc(),
                end_time: end_datetime.to_utc(),
                owner: user.clone(),
                participants: vec![user],
                game: game_opt,
            })
        }
        Err(e) => Err(ServerFnError::new(format!(
            "failed to create session: {}",
            e
        ))),
    }
}
