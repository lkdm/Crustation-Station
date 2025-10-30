mod detail;
mod layout;
mod list;

use crate::{
    app::{detail::DetailView, layout::Layout, list::ListView},
    features::{json::JsonParserFormatter, uuid::UUIDGeneratorEncoder},
};
use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

/// TODO: Implement IndexedDB storage for AppState
#[derive(Clone)]
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
        <Router>
            <Layout sidebar=ListView>
                <Routes fallback=|| "Not found">
                    // Because each view is behind a closure, they are lazy-created when the route
                    // is active.
                    <Route path=path!("/") view=move || view! { <DetailView><JsonParserFormatter/></DetailView> } />
                    <Route path=path!("/json") view=move || view! { <DetailView><JsonParserFormatter/></DetailView> } />
                    <Route path=path!("/uuid") view=move || view! { <DetailView><UUIDGeneratorEncoder/></DetailView> } />
                </Routes>
            </Layout>
        </Router>
    }
}
