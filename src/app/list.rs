mod navigation;

use leptos::prelude::*;
use leptos_router::components::A;
use leptos_shadcn_button::Button;

use crate::app::{AppState, list::navigation::Navigation};

#[component]
pub fn ListView() -> impl IntoView {
    view! {
        // Vertical space between top and bottom sidebar elements
        <div class="flex flex-col h-full justify-between border-r border-border">
            // Grouped elements at top of sidebar
            <div class="flex flex-col gap-2">
                <Branding />
                <Navigation />
            </div>
            <Config />
        </div>
    }
}

#[component]
pub fn Branding() -> impl IntoView {
    view! {
        <A href="/">
            <div class="flex flex-row gap-2 pl-4 pr-4 pt-2 pb-2 border-b border-border bg-card items-center text-primary">
                <span class="flex-none">"🦀"</span>
                <span class="flex-1 text-base font-semibold">"Crustacean Station"</span>
            </div>
        </A>
    }
}

#[component]
pub fn Config() -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState should be provided");
    let dark_mode = state.dark_mode;

    let handle_toggle_theme = {
        let dark_mode = dark_mode.clone();
        Callback::new(move |_| {
            dark_mode.set(!dark_mode.get());
        })
    };

    view! {
        <div class="flex flex-col gap-2 p-2">
            <Button on_click=handle_toggle_theme>
                {move || if dark_mode.get() {
                    "Switch to light mode"
                } else {
                    "Switch to dark mode"
                }}
            </Button>
        </div>
    }
}
