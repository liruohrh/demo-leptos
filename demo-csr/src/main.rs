use leptos::prelude::*;
use leptos::web_sys::console;

fn main() {
    mount_to_body(|| {
        view! {
            <div>
                <h1>"Hello, Leptos!"</h1>
                <p>"This is a simple Leptos application."</p>
                <p>"GOOD!!!😇"</p>
                <AButton name="AButton1" />
                <AButton name="AButton2" />
            </div>
        }
    })
}

#[component]
pub fn AButton(
    #[prop(into)] name: String,
    #[prop(default = false)] disabled: bool,
) -> impl IntoView {
    view! {
        <button
            disabled=disabled
            on:click=move |_| {
                console::log_1(&format!("{} clicked!", name.clone()).into());
            }
        >
            {name.clone()}
        </button>
    }
}
