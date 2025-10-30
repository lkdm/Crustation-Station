use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Branding() -> impl IntoView {
    view! {
        <div class="text-center">
            <A href="/"><h1 class="font-mono text-xs">"🦀 Crustacean Station"</h1></A>
        </div>
    }
}

#[component]
pub fn ListView() -> impl IntoView {
    view! {
        <Branding />
        "Sidebar"
    }
}
