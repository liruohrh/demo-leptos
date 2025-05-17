use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico" />
        <Router>
            <Routes fallback=|| "Page not found.">
                <Route path=StaticSegment("") view=Home />
            </Routes>
        </Router>
    }
}

#[component]
fn Home() -> impl IntoView {
    let (value, set_value) = signal(0);

    // thanks to https://tailwindcomponents.com/component/blue-buttons-example for the showcase layout
    view! {
        <Title text="Leptos + Tailwindcss" />
        <main>
            <div class="flex flex-col min-h-screen font-mono text-white bg-gradient-to-tl from-green-500 to-green-100">
                <div class="text-2xl text-center text-blue-500">"Leptos + Tailwindcss!!!"</div>
                <div class="flex flex-row-reverse flex-wrap m-auto">
                    <button
                        on:click=move |_| set_value.update(|value| *value += 1)
                        class="py-2 px-3 m-1 text-white bg-green-300 rounded border-l-2 border-b-4 border-green-300 shadow-lg"
                    >
                        "+"
                    </button>
                    <button class="py-2 px-3 m-1 text-red-300 bg-green-300 rounded border-l-2 border-b-4 border-green-300 shadow-lg">
                        {value}
                    </button>
                    <button
                        on:click=move |_| set_value.update(|value| *value -= 1)
                        class="py-2 px-3 m-1 text-white bg-green-300 rounded border-l-2 border-b-4 border-green-300 shadow-lg"
                        class:invisible=move || { value.get() < 1 }
                    >
                        "-"
                    </button>
                </div>
            </div>
        </main>
    }
}
