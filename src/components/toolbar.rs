use leptos::prelude::*;

#[component]
pub fn Toolbar(children: Children) -> impl IntoView {
    view! {
        <feature-toolbar class="flex flex-row items-center gap-6 h-12 mb-4">
            {children()}
        </feature-toolbar>
    }
}
