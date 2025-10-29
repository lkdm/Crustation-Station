mod detail;
mod layout;
mod list;

use crate::app::{detail::DetailView, layout::Layout, list::ListView};
use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Layout
            sidebar=move || view!{<ListView/>}
            detail=move || view!{<DetailView/>}
        />
    }
}
