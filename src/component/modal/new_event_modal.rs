use leptos::prelude::*;

use crate::obf_util::UrlParams;

#[component]
pub fn NewEventModal(url_params: UrlParams) -> impl IntoView {
    view! {
        <div class="absolute bottom-5 right-5">
            <button class="btn btn-xl btn-circle btn-secondary" onclick="my_modal_1.showModal()">
                <svg width="48" height="48" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor" className="size-6">
                    <path strokeLinecap="round" strokeLinejoin="round" d="M12 4.5v15m7.5-7.5h-15" />
                </svg>
            </button>
        </div>
        <dialog id="my_modal_1" class="modal modal">
        <div class="modal-box w-80">
            <h3 class="text-lg font-bold">Create Event</h3>
            <div class="modal-action mt-0 justify-center">
            <form method="dialog">
                <button class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2">{"✕"}</button>
                <fieldset class="fieldset w-full bg-base-200 border border-base-300 p-4 rounded-box">
                    <legend class="fieldset-legend">Event</legend>

                    <label class="fieldset-label">Title</label>
                    <input type="text" class="input" placeholder="Title" />

                    <label class="fieldset-label">Start Time</label>
                    <input type="time" class="input" />

                    <label class="fieldset-label">End Time</label>
                    <input type="time" class="input" />

                    <button class="btn btn-neutral mt-4">Login</button>
                </fieldset>
            </form>
            </div>
        </div>
        </dialog>
    }
}
