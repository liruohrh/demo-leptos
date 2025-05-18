use leptos::{logging, prelude::*, web_sys};
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment, WildcardSegment,
};

#[component]
pub fn App() -> impl IntoView {
    logging::log!("where do I run?");
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/tailwind-ssr-actix.css" />

        // sets the document title
        <Title text="Welcome to Leptos" />

        // content for this welcome page
        <Router>
            <main class="w-full h-full">
                <Routes fallback=move || "Not found.">
                    <Route path=StaticSegment("") view=HomePage />
                    <Route path=WildcardSegment("any") view=NotFound />
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let theme = RwSignal::new("forest");
    let toggle_theme = |theme| match theme {
        "light" => "dark",
        "dark" => "light",
        _ => "light",
    };
    let on_toggle_theme = move |_| {
        let new_theme = toggle_theme(theme.get());
        theme.set(new_theme);
        if let Some(htmlEl) = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|document| document.document_element())
        {
            let _ = htmlEl.set_attribute(
                "data-theme",
                if new_theme == "light" {
                    "lemonade"
                } else {
                    "forest"
                },
            );
        }
    };
    // Creates a reactive value to update the button
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;

    view! {
        <div class="flex flex-col gap-4 justify-center items-center w-full h-full">
            <h1>"Welcome to Leptos!"</h1>
            <button
                class="py-2 px-4 font-bold text-white bg-blue-500 rounded hover:bg-blue-700"
                on:click=on_click
            >
                "Click Me: "
                {count}
            </button>

            <button
                class="py-2 px-4 font-bold text-white bg-red-500 rounded hover:bg-red-700"
                on:click=on_toggle_theme
            >
                "Toggle Theme to "
                {move || toggle_theme(theme.get())}
            </button>
            <table class="table text-center text-yellow-400">
                <caption>Table of SB</caption>
                <colgroup>
                    <col class="w-40" />
                    <col class="w-40" />
                    <col class="w-60" />
                </colgroup>
                <tr class="space-x-4">
                    <th>Avater</th>
                    <th>Name</th>
                    <th>Age</th>
                </tr>
                <tr class="space-x-4 hover:bg-base-200">
                    <td>
                        <div class="flex justify-center avatar">
                            <div class="w-24 rounded">
                                <img src="https://img.daisyui.com/images/profile/demo/batperson@192.webp" />
                            </div>
                        </div>
                    </td>
                    <td>Alice</td>
                    <td>20</td>
                </tr>
                <tr class="space-x-4 hover:bg-base-200">
                    <td>
                        <div class="flex justify-center avatar">
                            <div class="w-24 rounded">
                                <img src="https://img.daisyui.com/images/profile/demo/yellingcat@192.webp" />
                            </div>
                        </div>
                    </td>
                    <td>Bob</td>
                    <td>30</td>
                </tr>
            </table>
        </div>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! { <h1>"Not Found"</h1> }
}
