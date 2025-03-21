use leptos::prelude::*;

#[component]
pub fn NavBar() -> impl IntoView {
    let themes = vec![
        "retro",
        "dracula",
        "cupcake",
        "synthwave",
        "valentine",
        "lofi",
        "business",
        "caramellatte",
        "walkingspace",
    ];

    view! {
        <div class="absolute z-4 top-0 navbar bg-base-100 shadow-sm">
            <div class="flex-1">
                <a class="btn btn-ghost text-xl">Game Tonite!</a>
            </div>
            <div class="flex-none">
                <div class="dropdown dropdown-end">
                    <div tabindex="0" role="button" class="btn m-1">
                        Theme
                        <svg
                        width="12px"
                        height="12px"
                        class="inline-block h-2 w-2 fill-current opacity-60"
                        xmlns="http://www.w3.org/2000/svg"
                        viewBox="0 0 2048 2048">
                        <path d="M1799 349l242 241-1017 1017L7 590l242-241 775 775 775-775z"></path>
                        </svg>
                    </div>
                    <ul tabindex="0" class="dropdown-content bg-base-300 rounded-box z-1 w-52 p-3 shadow-2xl">
                        {
                            themes.into_iter().map(|t| view! {
                                <li class="">
                                <input
                                    type="radio"
                                    name="theme-dropdown"
                                    class="theme-controller w-full mt-[3px] mb-[3px] btn btn-sm border-0 btn-block btn-ghost justify-start"
                                    aria-label={ t.to_string() }
                                    value={ t.to_string() }
                                />
                                </li>
                            }).collect_view()
                        }
                    </ul>
                </div>
            </div>
        </div>
    }
}
