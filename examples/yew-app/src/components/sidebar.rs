// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::types::BumpPart;
use input_rs::yew::Input;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SidebarProps {
    pub active_part: BumpPart,
    pub on_part_change: Callback<BumpPart>,
    pub custom_part: UseStateHandle<String>,
    pub custom_part_valid: UseStateHandle<bool>,
    pub parse_override: UseStateHandle<String>,
    pub parse_valid: UseStateHandle<bool>,
    pub serialize_override: UseStateHandle<String>,
    pub sidebar_open: bool,
    pub on_close: Callback<()>,
}

fn sidebar_content(
    props: &SidebarProps,
    custom_ref: NodeRef,
    parse_ref: NodeRef,
    serialize_ref: NodeRef,
) -> Html {
    let on_close = {
        let cb = props.on_close.clone();
        Callback::from(move |_: MouseEvent| cb.emit(()))
    };

    let show_custom = matches!(&props.active_part, BumpPart::Custom(_));

    html! {
        <nav class="sidebar-nav" aria-label="Version bump configuration" id="bump-sidebar">
            <div style="display:flex;align-items:center;justify-content:space-between;">
                <h2
                    style="font-size:.7rem;font-weight:700;text-transform:uppercase;\
                           letter-spacing:.06em;color:var(--color-subtle);"
                >
                    { "Configuration" }
                </h2>
                <button
                    class="btn btn-ghost"
                    onclick={on_close}
                    aria-label="Close configuration panel"
                    style="padding:.2rem .5rem;font-size:.75rem;"
                >
                    { "✕" }
                </button>
            </div>
            <section class="sidebar-section" aria-labelledby="part-label">
                <h3 id="part-label" class="sidebar-label">{ "Bump Part" }</h3>
                <div class="preset-group" role="radiogroup" aria-labelledby="part-label">
                    { for BumpPart::standard().into_iter().map(|p| {
                        let active = p == props.active_part;
                        let cb = props.on_part_change.clone();
                        let pc = p.clone();
                        let on_click = Callback::from(move |_: MouseEvent| cb.emit(pc.clone()));
                        let btn_class = if active {
                            "preset-btn preset-btn--active"
                        } else {
                            "preset-btn"
                        };
                        html! {
                            <button
                                key={p.label()}
                                class={btn_class}
                                onclick={on_click}
                                role="radio"
                                aria-checked={active.to_string()}
                                id={format!("part-{}", p.label())}
                                title={p.description()}
                            >
                                <span class="preset-icon" aria-hidden="true">{ p.emoji() }</span>
                                <div style="display:flex;flex-direction:column;align-items:flex-start;min-width:0;">
                                    <span style="font-size:.82rem;font-weight:500;line-height:1;">
                                        { p.label() }
                                    </span>
                                    <span style="font-size:.68rem;opacity:.5;margin-top:2px;line-height:1.3;">
                                        { p.description() }
                                    </span>
                                </div>
                            </button>
                        }
                    }) }
                    <button
                        class={if show_custom { "preset-btn preset-btn--active" } else { "preset-btn" }}
                        onclick={{
                            let cb = props.on_part_change.clone();
                            let cur = (*props.custom_part).clone();
                            Callback::from(move |_: MouseEvent| {
                                cb.emit(BumpPart::Custom(cur.clone()))
                            })
                        }}
                        role="radio"
                        aria-checked={show_custom.to_string()}
                        id="part-custom"
                        title="A custom named version component"
                    >
                        <span class="preset-icon" aria-hidden="true">{ "🔧" }</span>
                        <span style="font-size:.82rem;font-weight:500;">{ "Custom" }</span>
                    </button>
                </div>
                if show_custom {
                    <div style="margin-top:.5rem;">
                        <Input
                            r#type="text"
                            label="Part name"
                            handle={props.custom_part.clone()}
                            name="custom_part"
                            r#ref={custom_ref}
                            placeholder="e.g. pre, rc, dev"
                            input_class=""
                            field_class=""
                            label_class="sidebar-label"
                            error_class="input-error"
                            error_message="Part name cannot be empty"
                            valid_handle={props.custom_part_valid.clone()}
                            validate_function={Callback::from(|v: String| !v.trim().is_empty())}
                            id="custom-part-input"
                        />
                    </div>
                }
            </section>
            <section class="sidebar-section" aria-labelledby="advanced-label">
                <h3 id="advanced-label" class="sidebar-label">{ "Advanced Overrides" }</h3>
                <p
                    style="font-size:.7rem;color:var(--color-subtle);line-height:1.5;margin-bottom:.25rem;"
                >
                    { "Override the parse regex and serialize format. Leave blank to use defaults." }
                </p>
                <Input
                    r#type="text"
                    label="Parse regex"
                    handle={props.parse_override.clone()}
                    name="parse_override"
                    r#ref={parse_ref}
                    placeholder="(?P<major>\\d+)\\.(?P<minor>\\d+)\\.(?P<patch>\\d+)"
                    input_class=""
                    field_class=""
                    label_class="sidebar-label"
                    error_class="input-error"
                    error_message="Enter a valid regex with named groups"
                    valid_handle={props.parse_valid.clone()}
                    validate_function={Callback::from(|_: String| true)}
                    id="parse-override-input"
                />
                <div style="margin-top:.5rem;">
                    <Input
                        r#type="text"
                        label="Serialize format"
                        handle={props.serialize_override.clone()}
                        name="serialize_override"
                        r#ref={serialize_ref}
                        placeholder="{major}.{minor}.{patch}"
                        input_class=""
                        field_class=""
                        label_class="sidebar-label"
                        error_class=""
                        error_message=""
                        valid_handle={props.parse_valid.clone()}
                        validate_function={Callback::from(|_: String| true)}
                        id="serialize-override-input"
                    />
                </div>
            </section>
            <div class="sidebar-footer">
                { "Powered by " }
                <strong style="color:var(--color-text);">{ "bump2version" }</strong>
                { " · All bumping runs locally via WebAssembly. No data leaves your browser." }
            </div>
        </nav>
    }
}

#[function_component(Sidebar)]
pub fn sidebar(props: &SidebarProps) -> Html {
    let custom_ref = use_node_ref();
    let parse_ref = use_node_ref();
    let serialize_ref = use_node_ref();

    let content = sidebar_content(props, custom_ref, parse_ref, serialize_ref);

    html! {
        <>
            <aside
                class={if props.sidebar_open {
                    "app-sidebar app-sidebar--mobile-open"
                } else {
                    "app-sidebar"
                }}
                aria-label="Configuration panel"
                id="bump-sidebar-aside"
            >
                { content }
            </aside>
            if props.sidebar_open {
                <div
                    class="sidebar-backdrop"
                    onclick={{
                        let cb = props.on_close.clone();
                        Callback::from(move |_: MouseEvent| cb.emit(()))
                    }}
                    aria-hidden="true"
                />
            }
        </>
    }
}
