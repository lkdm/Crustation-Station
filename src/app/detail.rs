use leptos::prelude::*;

#[component]
pub fn DetailView(children: Children) -> impl IntoView {
    view! {
        <>
            {children()}
        </>
    }
}
