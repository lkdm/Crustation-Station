use leptos::prelude::*;

#[component]
pub fn DetailView(
    title: String,
    children: Children
) -> impl IntoView {
    view! {
        <>
            <h1>{title}</h1>
            <div style="height: calc(100vh - 64px)">
                {children()}
            </div>
        </>
    }
}
