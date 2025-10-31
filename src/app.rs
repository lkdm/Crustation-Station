mod detail;
mod layout;
mod list;
pub mod state;

use crate::{
    app::{detail::DetailView, layout::Layout, list::ListView, state::AppState},
    features::{json::JsonParserFormatter, uuid::UUIDGeneratorEncoder},
};
use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

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
                    <Route path=path!("/") view=move || view! { <DetailView title="Json Parser and Formatter".to_string()><JsonParserFormatter/></DetailView> } />
                    <Route path=path!("/json") view=move || view! { <DetailView title="Json Parser and Formatter".to_string()><JsonParserFormatter/></DetailView> } />
                    <Route path=path!("/uuid") view=move || view! { <DetailView title="UUID/ULID Generator and Encoder".to_string()><UUIDGeneratorEncoder/></DetailView> } />
                </Routes>
            </Layout>
        </Router>
    }
}
