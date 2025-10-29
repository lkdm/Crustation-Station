use leptos::prelude::*;

#[component]
pub fn Layout(sidebar: impl IntoView, detail: impl IntoView) -> impl IntoView {
    view! {
        <div class="flex flex-col md:flex-row h-screen">
            <aside class="bg-gray-800 text-white w-full md:w-60 h-20 md:h-full overflow-y-auto bg-red-500">
                {sidebar.into_view()}
            </aside>

            <main class="flex-1 bg-gray-100 p-4 overflow-auto">
                {detail.into_view()}
            </main>
        </div>
    }
}
