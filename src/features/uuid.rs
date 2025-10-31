use crate::components::{copy_button::CopyButton, form::textarea_attrs, toolbar::Toolbar};
use leptos::prelude::*;
use leptos_shadcn_button::Button;
use leptos_shadcn_checkbox::Checkbox;
use leptos_shadcn_combobox::{Combobox, ComboboxOption};
use leptos_shadcn_input::Input;
use leptos_shadcn_label::Label;
use std::str::FromStr;
use strum::{Display, EnumString};
use uuid::Uuid;

// Copy this in to test:
// {"foo":"bar","baz":["qux"]}

#[derive(EnumString, Display, Clone, Debug, PartialEq)]
#[strum(serialize_all = "UPPERCASE")]
enum Kind {
    V4,
    // V7,
}

impl Kind {
    pub fn label(&self) -> String {
        match &self {
            Kind::V4 => "UUID v4",
        }
        .to_string()
    }
}

fn generate(n: u32, kind: Kind) -> Result<Vec<String>, String> {
    let cl = match kind {
        Kind::V4 => || Uuid::new_v4(),
        // Kind::V7 => || Uuid::new_v7(Timestamp::now()),
    };

    let uuids = (0..n).map(|_| cl().to_string()).collect::<Vec<_>>();

    Ok(uuids)
}

#[component]
pub fn UUIDGeneratorEncoder() -> impl IntoView {
    let (kind, set_kind) = signal::<Kind>(Kind::V4);
    let (number, set_number) = signal::<u32>(10);
    let (result, set_result) = signal::<Result<String, String>>(Ok("".to_string()));
    let (lowercase, set_lowercase) = signal::<bool>(false);

    let handle_generate_ids = {
        let kind = kind.clone();
        let number = number.clone();
        let lowercase = lowercase.clone();
        move || {
            let n = number.get();
            let k = kind.get();
            let l = lowercase.get();
            match (generate(n, k), l) {
                // By default it outputs lower case
                (Ok(uuids), true) => set_result.set(Ok(uuids.join("\n"))),
                // To uppercase
                (Ok(uuids), false) => set_result.set(Ok(uuids.join("\n").to_uppercase())),
                (Err(err), _) => set_result.set(Err(err)),
            }
        }
    };

    let handle_change_result_case = {
        let result = result.clone();
        let lowercase = lowercase.clone();
        move || {
            let new_result = match lowercase.get() {
                false => result.get().unwrap_or_default().to_uppercase(),
                true => result.get().unwrap_or_default().to_lowercase(),
            };
            set_result.set(Ok(new_result));
        }
    };

    view! {
        <div class="flex flex-row gap-x-4 h-full">
            <feature-result class="flex flex-col flex-1 h-full w-full min-w-0">
                <Toolbar>
                    <Label>
                        "Generate new IDs"
                    </Label>
                    <div class="w-1/3 flex flex-row gap-4 justify-end items-center">
                        <Combobox
                            options=vec![
                                ComboboxOption::new(Kind::V4.label(), Kind::V4.to_string())
                            ]
                            value=Some(kind.get().to_string())
                            on_change=Callback::new(move |val: String| {
                                set_kind.set(Kind::from_str(&val).unwrap())
                            })
                        />
                        <Label>"x"</Label>
                        <Input
                            value=Some(number.get().to_string())
                            on_change=Callback::new(move |val: String| {
                                if let Ok(parsed) = val.parse::<u32>() {
                                    set_number.set(parsed);
                                } else {
                                    // TODO: Handle invalid input better
                                    set_number.set(0);
                                }
                            })
                        />
                    </div>
                </Toolbar>
                <Toolbar>
                    <Button
                        on_click=handle_generate_ids.clone()
                    >
                        Generate
                    </Button>
                    <CopyButton
                        disabled=Signal::derive(move || result.get().is_err())
                        text_to_copy=Signal::derive(move || result.get().unwrap_or("".to_string()))
                    />
                    <Button
                        on_click=move || set_result.set(Ok("".to_string()))
                    >
                        Clear
                    </Button>
                    <div class="flex flex-row gap-2 justify-end items-center">
                        <Checkbox
                            on_change=Callback::new(move |val: bool| {
                                set_lowercase.set(val);
                                handle_change_result_case();
                            })
                        />
                        <Label>lowercase</Label>
                    </div>
                </Toolbar>
                <textarea
                    {..textarea_attrs("flex-1 w-full resize-none font-mono")}
                    prop:value=move || result.get().unwrap()
                    on:input:target=move |ev| {
                        set_result.set(Ok(ev.target().value()))
                    }
                >
                    {result}
                </textarea>
            </feature-result>
        </div>
    }
}
