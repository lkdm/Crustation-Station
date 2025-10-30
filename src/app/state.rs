use leptos::prelude::*;

/// TODO: Implement IndexedDB storage for AppState
#[derive(Clone)]
pub struct AppState {
    pub dark_mode: RwSignal<bool>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            dark_mode: RwSignal::new(false),
        }
    }
}
