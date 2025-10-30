use gloo_storage::{LocalStorage, Storage};
use leptos::prelude::*;

/// TODO: Implement IndexedDB storage for AppState
#[derive(Clone)]
pub struct AppState {
    pub dark_mode: RwSignal<bool>,
}

impl AppState {
    pub fn new() -> Self {
        let initial_dark_mode = LocalStorage::get::<bool>("dark_mode").unwrap_or(false);
        let dark_mode = RwSignal::new(initial_dark_mode);
        // Whenever dark_mode changes, update storage
        Effect::new(move |_| {
            let value = dark_mode.get();
            LocalStorage::set("dark_mode", value).expect("Failed to save dark_mode");
        });

        Self { dark_mode }
    }
}
