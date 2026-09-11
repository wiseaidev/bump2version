// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::components::history_entry::HistoryEntry;
use crate::types::{BumpPart, BumpRecord};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct HistoryPanelProps {
    pub records: Vec<BumpRecord>,
}

fn empty_state() -> Html {
    let modes = BumpPart::standard();
    html! {
        <div class="history-empty">
            <div class="empty-icon" aria-hidden="true">{ "⬆️" }</div>
            <div>
                <h2 class="empty-title">{ "Welcome to bump2version" }</h2>
                <p class="empty-desc">
                    { "Enter a version string below and choose a bump type to get started. \
                    All computation runs locally in your browser via WebAssembly, \
                    no server, no network." }
                </p>
            </div>
            <ul
                style="display:grid;grid-template-columns:repeat(auto-fill,minmax(140px,1fr));\
                       gap:.75rem;width:100%;list-style:none;"
                aria-label="Available bump types"
            >
                { for modes.iter().map(|p| html! {
                    <li
                        key={p.label()}
                        style="background:var(--color-surface);border:1px solid var(--color-border);\
                               border-radius:var(--radius-lg);padding:.875rem;text-align:left;"
                    >
                        <span style="font-size:1.4rem;" aria-hidden="true">{ p.emoji() }</span>
                        <p style="font-size:.85rem;font-weight:600;color:var(--color-text);margin-top:.375rem;">
                            { p.label() }
                        </p>
                        <p style="font-size:.72rem;color:var(--color-text-dim);margin-top:.2rem;line-height:1.4;">
                            { p.description() }
                        </p>
                    </li>
                }) }
            </ul>
        </div>
    }
}

#[function_component(HistoryPanel)]
pub fn history_panel(props: &HistoryPanelProps) -> Html {
    let bottom_ref = use_node_ref();
    let len = props.records.len();

    {
        let bottom_ref = bottom_ref.clone();
        use_effect_with(len, move |_| {
            if let Some(el) = bottom_ref.cast::<web_sys::Element>() {
                el.scroll_into_view();
            }
            move || {}
        });
    }

    html! {
        <section
            class="history-panel"
            aria-label="Bump history"
            aria-live="polite"
            aria-relevant="additions"
            id="history-panel"
        >
            if props.records.is_empty() {
                { empty_state() }
            } else {
                <ol
                    class="history-list"
                    aria-label={format!("{} bump{} recorded", len, if len == 1 { "" } else { "s" })}
                    reversed=true
                >
                    { for props.records.iter().map(|r| html! {
                        <li key={r.id}>
                            <HistoryEntry record={r.clone()} />
                        </li>
                    }) }
                </ol>
                <div ref={bottom_ref} id="history-bottom" aria-hidden="true" />
            }
        </section>
    }
}
