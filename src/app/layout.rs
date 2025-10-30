use crate::app::AppState;
use leptos::prelude::*;

#[component]
pub fn Layout(sidebar: impl IntoView, children: Children) -> impl IntoView {
    let state: AppState = use_context().expect("AppState not found");

    // TODO: Use the <Html/> component instead of an effect
    // https://book.leptos.dev/metadata.html
    Effect::new(move |_| {
        let dark = state.dark_mode.get();

        let document = window().document().unwrap();
        let body = document.body().unwrap();

        if dark {
            body.class_list().add_1("dark").unwrap();
        } else {
            body.class_list().remove_1("dark").unwrap();
        }
    });

    view! {
        <div class="bg-background text-foreground transition-colors duration-500 flex flex-col md:flex-row h-screen">
            <aside class="w-full md:w-72 md:h-full overflow-y-auto">
                {sidebar.into_view()}
            </aside>

            <main class="flex-1 p-4 overflow-auto h-full">
                {children()}
            </main>
        </div>
    }
}
