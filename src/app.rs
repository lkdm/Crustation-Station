mod detail;
mod layout;
mod list;

use crate::app::{detail::DetailView, layout::Layout, list::ListView};
use leptos::prelude::*;

/// TODO: Implement IndexedDB storage for AppState
pub struct AppState {
    pub dark_mode: RwSignal<bool>,
}

impl AppState {
    fn new() -> Self {
        Self {
            dark_mode: RwSignal::new(false),
        }
    }
}

#[component]
pub fn App() -> impl IntoView {
    let state = AppState::new();
    provide_context(state);
    view! {
        <Layout
            sidebar=move || view!{<ListView/>}
            detail=move || view!{<DetailView/>}
        />
    }
}
