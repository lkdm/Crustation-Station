use leptos::prelude::*;
use leptos_meta::*;

#[component]
pub fn DetailView(title: String, children: Children) -> impl IntoView {
    view! {
        <>
            <h1 class="font-semibold mb-4">{title.clone()}</h1>
            <Title text=title />
            <div style="height: calc(100vh - 72px)">
                {children()}
            </div>
        </>
    }
}
