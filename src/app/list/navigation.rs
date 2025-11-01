use leptos::prelude::*;
use leptos_router::components::A;

use crate::app::route;

#[derive(Clone)]
struct NavItem {
    title: &'static str,
    href: String,
}

#[component]
pub fn Navigation() -> impl IntoView {
    let items = vec![
        NavItem {
            title: "JSON Formatter",
            href: route("json"),
        },
        NavItem {
            title: "UUID / ULID Tool",
            href: route("uuid"),
        },
    ];

    view! {
        <ul class="flex flex-col gap-1 p-2">
            {items.into_iter()
                .map(|item| view! {
                    <A
                        href=item.href
                        // TODO: I can't get :active to work with <A/> for some reason?
                        attr:class="block rounded-md px-3 py-2 text-sm text-foreground hover:bg-secondary transition-colors"
                    >
                        {item.title}
                    </A>
                }).collect_view()}
        </ul>
    }
}
