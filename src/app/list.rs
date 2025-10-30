use leptos::prelude::*;
use leptos_router::components::A;
use leptos_shadcn_label::Label;
use leptos_shadcn_switch::Switch;

use crate::app::AppState;

#[component]
pub fn Branding() -> impl IntoView {
    view! {
        <A href="/">
            <div class="flex flex-row gap-2 p-2 border-b items-center">
                <span class="flex-none">"🦀"</span>
                <span class="flex-1 text-base font-semibold">"Crustacean Station"</span>
            </div>
        </A>
    }
}

#[component]
pub fn ListView() -> impl IntoView {
    view! {
        // Vertical space between top and bottom sidebar elements
        <div class="flex flex-col h-full justify-between p-2">
            // Grouped elements at top of sidebar
            <div class="flex flex-col gap-2">
                <Branding />
                <nav>"Sidebar"</nav>
            </div>
            <Config />
        </div>
    }
}

#[component]
pub fn Config() -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState should be provided");

    view! {
        <div class="flex flex-col gap-2 p-2">
            <Label>"Theme"</Label>
            <Switch
                checked=state.dark_mode.read_only()
                on_change=Callback::new(move |val| state.dark_mode.set(val))
            />
        </div>
    }
}
