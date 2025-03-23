use leptos::{html::Dialog, logging::log, prelude::*};

/**
 * Modal form to create a new event
 */
#[component]
pub fn NewEventModal() -> impl IntoView {
    let e = NodeRef::<Dialog>::new();
    let (error_status, set_error_status) = signal(false);

    // handle ActionForm
    let create_event = ServerAction::<CreateEvent>::new();
    let server_res = create_event.value();
    Effect::new(move || match server_res() {
        Some(Ok(())) => {
            set_error_status(false);
            e.get().unwrap().close()
        }
        Some(Err(_)) => {
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
                            <input type="text" class="input" placeholder="Title" name="title" />

                            <label class="fieldset-label">Start Time</label>
                            <input type="time" class="input" name="start" />

                            <label class="fieldset-label">End Time</label>
                            <input type="time" class="input" name="end" />

                            <button type="submit" class="btn btn-neutral mt-4">Create</button>
                        </fieldset>
                    </ActionForm>
                </div>
            </div>
        </dialog>
    }
}

#[server]
pub async fn create_event(title: String, start: String, end: String) -> Result<(), ServerFnError> {
    log!("{} {} {}", title, start, end);
    Ok(())
}
