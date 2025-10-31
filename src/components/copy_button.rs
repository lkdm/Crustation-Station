use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;
use leptos_shadcn_button::{Button, ButtonSize};
use wasm_bindgen_futures::spawn_local;

use crate::components::form::button_attrs;

/// CopyButton
///
/// Provides a UI button allowing the user to copy the value to their clipboard.
#[component]
pub fn CopyButton(
    #[prop(into)] disabled: Signal<bool>,
    #[prop(into)] text_to_copy: Signal<String>,
) -> impl IntoView {
    let (copied, set_copied) = signal(false);
    let button_class = Signal::derive(move || {
        if disabled.get() {
            "w-24 bg-secondary text-muted-foreground cursor-not-allowed".to_string()
        } else {
            "w-24".to_string()
        }
    });

    // When button is clicked, copy the value to the clipboard
    let on_click = move || {
        let value_to_copy = text_to_copy.get(); // Get the value to copy
        let window = web_sys::window().unwrap();
        let navigator = window.navigator();
        let clipboard = navigator.clipboard();

        // Write the value to clipboard and update the copied state
        let _ = clipboard.write_text(&value_to_copy);

        // Set the copied state to true for a short period, e.g., 1 second
        set_copied.set(true);

        // Reset the "Copied!" state after 1 second
        let cloned_set_copied = set_copied.clone();
        spawn_local(async move {
            // Wait for a second before resetting the copied state
            TimeoutFuture::new(1000).await;
            cloned_set_copied.set(false);
        });
    };

    view! {
        <button
            {..button_attrs(move || button_class.get().into())}
            disabled=disabled
            on:click=move |_| on_click()
        >
            {move || if copied.get() { view! { "Copied!" } } else { view! { "Copy" } }}
        </button>
    }
}

// on:click=move |_| setter.update(|value| *value = !*value)
