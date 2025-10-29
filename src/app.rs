mod detail;
mod layout;
mod list;

use crate::app::{detail::DetailView, layout::Layout, list::ListView};
use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

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
        <Router>
            <Layout sidebar=move || view! { <ListView/> }>
                <Routes fallback=|| "Not found">
                    <Route path=path!("/") view=move || view! { <DetailView><MyCalculator1/></DetailView> } />
                    <Route path=path!("/another") view=move || view! { <DetailView><AnotherTool/></DetailView> } />
                </Routes>
            </Layout>
        </Router>
    }
}

#[component]
pub fn MyCalculator1() -> impl IntoView {
    view! {
        "MyCalculator1"
    }
}

#[component]
pub fn AnotherTool() -> impl IntoView {
    view! {
        "AnotherTool"
    }
}
