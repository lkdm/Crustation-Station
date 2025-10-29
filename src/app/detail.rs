use leptos::prelude::*;

#[component]
pub fn DetailView(children: Children) -> impl IntoView {
    view! {
        <section class="p-6">
            {children()}
        </section>
    }
}
