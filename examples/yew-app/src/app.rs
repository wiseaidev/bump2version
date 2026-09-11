// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::components::header::Header;
use crate::components::history_panel::HistoryPanel;
use crate::components::input_bar::InputBar;
use crate::components::sidebar::Sidebar;
use crate::types::{BumpPart, BumpRecord};
use crate::version_ops::execute_bump;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let records = use_state(Vec::<BumpRecord>::new);
    let next_id = use_state(|| 0usize);
    let sidebar_open = use_state(|| false);
    let active_part = use_state(|| BumpPart::Patch);
    let custom_part = use_state(String::new);
    let custom_part_valid = use_state(|| true);
    let parse_override = use_state(String::new);
    let parse_valid = use_state(|| true);
    let serialize_override = use_state(String::new);
    let announce = use_state(String::new);
    let error = use_state(String::new);

    let on_part_change = {
        let active_part = active_part.clone();
        Callback::from(move |p: BumpPart| active_part.set(p))
    };

    let on_bump = {
        let records = records.clone();
        let next_id = next_id.clone();
        let active_part = active_part.clone();
        let custom_part = custom_part.clone();
        let parse_override = parse_override.clone();
        let serialize_override = serialize_override.clone();
        let announce = announce.clone();
        let error = error.clone();

        Callback::from(move |version_str: String| {
            let part = match &*active_part {
                BumpPart::Custom(_) => {
                    let cp = (*custom_part).trim().to_string();
                    if cp.is_empty() {
                        error.set("Custom part name cannot be empty".into());
                        return;
                    }
                    BumpPart::Custom(cp)
                }
                p => p.clone(),
            };

            match execute_bump(
                &version_str,
                &part.label(),
                &*parse_override,
                &*serialize_override,
            ) {
                Ok(result) => {
                    let id = *next_id;
                    let record = BumpRecord {
                        id,
                        old_version: version_str.clone(),
                        new_version: result.new_version.clone(),
                        part: part.clone(),
                    };
                    announce.set(format!(
                        "Version bumped from {} to {}",
                        version_str, result.new_version
                    ));
                    error.set(String::new());
                    next_id.set(id + 1);
                    let mut list = (*records).clone();
                    list.insert(0, record);
                    records.set(list);
                }
                Err(e) => error.set(e),
            }
        })
    };

    let on_sidebar_toggle = {
        let sidebar_open = sidebar_open.clone();
        Callback::from(move |_: ()| sidebar_open.set(!*sidebar_open))
    };

    let on_sidebar_close = {
        let sidebar_open = sidebar_open.clone();
        Callback::from(move |_: ()| sidebar_open.set(false))
    };

    html! {
        <div id="app" aria-label="bump2version application">
            <a href="#main-content" class="skip-link">{ "Skip to main content" }</a>
            <span class="sr-only" aria-live="polite" aria-atomic="true" role="status">
                { (*announce).clone() }
            </span>
            <Header sidebar_open={*sidebar_open} on_sidebar_toggle={on_sidebar_toggle} />
            <div class="layout-body">
                <Sidebar
                    active_part={(*active_part).clone()}
                    on_part_change={on_part_change}
                    custom_part={custom_part.clone()}
                    custom_part_valid={custom_part_valid.clone()}
                    parse_override={parse_override.clone()}
                    parse_valid={parse_valid.clone()}
                    serialize_override={serialize_override.clone()}
                    sidebar_open={*sidebar_open}
                    on_close={on_sidebar_close}
                />
                <main class="app-main" id="main-content" aria-label="Version bumper workspace">
                    <div class="mobile-toolbar">
                        <button
                            class="btn btn-ghost"
                            onclick={Callback::from({
                                let sb = sidebar_open.clone();
                                move |_| sb.set(!*sb)
                            })}
                            aria-label="Toggle configuration panel"
                            aria-expanded={sidebar_open.to_string()}
                            aria-controls="bump-sidebar"
                        >
                            { "⚙ Configure" }
                        </button>
                    </div>
                    <HistoryPanel records={(*records).clone()} />
                    <InputBar on_bump={on_bump} error={(*error).clone()} />
                </main>
            </div>
        </div>
    }
}
