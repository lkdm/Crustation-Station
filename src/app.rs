mod detail;
mod layout;
mod list;
pub mod state;

use crate::{
    app::{detail::DetailView, layout::Layout, list::ListView, state::AppState},
    features::{json::JsonParserFormatter, uuid::UUIDGeneratorEncoder},
};
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

pub fn base_route() -> &'static str {
    #[cfg(debug_assertions)]
    {
        "/"
    }

    #[cfg(not(debug_assertions))]
    {
        "/Crustacean-Station"
    }
}

pub fn route(path: &str) -> String {
    let base = base_route().trim_end_matches('/');
    let path = path.trim_start_matches('/');
    format!("{}/{}", base, path)
}

#[component]
pub fn App() -> impl IntoView {
    let state = AppState::new();
    provide_context(state);

    let formatter = |text| format!("{text} — Crustacean Station");

    view! {
        <Layout sidebar=ListView>
            <Title formatter/>
            <Meta charset="utf-8"/>
            <Meta name="author" content="Luke M"/>
            <Meta name="description" content="Developer tools on the web built in Rust"/>

            <Router base=base_route()>
                <Routes fallback=|| "Not found">
                    // Because each view is behind a closure, they are lazy-created when the route
                    // is active.
                    <Route path=path!("/") view=move || view! { <DetailView title="Json Parser and Formatter".to_string()><JsonParserFormatter/></DetailView> } />
                    <Route path=path!("/json") view=move || view! { <DetailView title="Json Parser and Formatter".to_string()><JsonParserFormatter/></DetailView> } />
                    <Route path=path!("/uuid") view=move || view! { <DetailView title="UUID/ULID Generator and Encoder".to_string()><UUIDGeneratorEncoder/></DetailView> } />
                </Routes>
            </Router>
        </Layout>
    }
}
