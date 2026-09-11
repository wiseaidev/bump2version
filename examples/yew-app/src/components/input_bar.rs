// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use input_rs::yew::Input;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct InputBarProps {
    pub on_bump: Callback<String>,
    pub error: String,
}

fn validate_version(v: String) -> bool {
    !v.trim().is_empty()
        && v.trim()
            .split('.')
            .all(|part| part.parse::<u64>().is_ok() || !part.is_empty())
}

fn do_bump(input_ref: &NodeRef, on_bump: &Callback<String>) {
    if let Some(el) = input_ref.cast::<HtmlInputElement>() {
        let val = el.value();
        if !val.trim().is_empty() {
            on_bump.emit(val.trim().to_string());
        }
    }
}

#[function_component(InputBar)]
pub fn input_bar(props: &InputBarProps) -> Html {
    let input_ref = use_node_ref();
    let input_handle = use_state(|| "1.0.0".to_string());
    let input_valid = use_state(|| true);

    {
        let r = input_ref.clone();
        use_effect_with((), move |_| {
            if let Some(el) = r.cast::<HtmlInputElement>() {
                el.set_value("1.0.0");
            }
            || ()
        });
    }

    let make_bump_cb = |on_bump: Callback<String>, r: NodeRef| {
        Callback::from(move |_: MouseEvent| do_bump(&r, &on_bump))
    };

    let on_patch = make_bump_cb(props.on_bump.clone(), input_ref.clone());
    let on_minor = make_bump_cb(props.on_bump.clone(), input_ref.clone());
    let on_major = make_bump_cb(props.on_bump.clone(), input_ref.clone());

    let on_keydown = {
        let r = input_ref.clone();
        let cb = props.on_bump.clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" && !e.shift_key() {
                e.prevent_default();
                do_bump(&r, &cb);
            }
        })
    };

    html! {
        <div class="input-bar" role="region" aria-label="Version input and bump controls">
            <div class="input-bar-inner" onkeydown={on_keydown}>
                <div class="input-field-wrap">
                    <label for="version-input" class="version-label">{ "Current Version" }</label>
                    <Input
                        r#type="text"
                        label=""
                        handle={input_handle}
                        name="version"
                        r#ref={input_ref}
                        placeholder="e.g. 1.2.3"
                        input_class="version-input"
                        field_class=""
                        error_class="input-error"
                        error_message="Enter a valid version string"
                        valid_handle={input_valid}
                        validate_function={Callback::from(validate_version)}
                        id="version-input"
                    />
                    if !props.error.is_empty() {
                        <p class="input-error" role="alert" aria-live="assertive">
                            { &props.error }
                        </p>
                    }
                </div>
                <div class="bump-buttons" role="group" aria-label="Bump type buttons">
                    <button
                        class="btn btn-patch"
                        onclick={on_patch}
                        aria-label="Bump patch version"
                        id="btn-patch"
                        type="button"
                    >
                        { "🩹 Patch" }
                    </button>
                    <button
                        class="btn btn-minor"
                        onclick={on_minor}
                        aria-label="Bump minor version"
                        id="btn-minor"
                        type="button"
                    >
                        { "✨ Minor" }
                    </button>
                    <button
                        class="btn btn-major"
                        onclick={on_major}
                        aria-label="Bump major version"
                        id="btn-major"
                        type="button"
                    >
                        { "🚀 Major" }
                    </button>
                </div>
            </div>
            <p class="input-hint" aria-hidden="true">
                { "Enter to bump patch  •  All processing runs locally in WebAssembly" }
            </p>
        </div>
    }
}
