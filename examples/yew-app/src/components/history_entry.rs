// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::components::part_badge::PartBadge;
use crate::types::BumpRecord;
use yew::prelude::*;

/// Props for [`HistoryEntry`].
#[derive(Properties, PartialEq)]
pub struct HistoryEntryProps {
    /// The bump record to display.
    pub record: BumpRecord,
}

#[function_component(HistoryEntry)]
pub fn history_entry(props: &HistoryEntryProps) -> Html {
    let label = format!(
        "Version bumped from {} to {} using {} bump",
        props.record.old_version,
        props.record.new_version,
        props.record.part.label()
    );

    html! {
        <article class="history-entry" aria-label={label}>
            <div
                class="entry-avatar entry-avatar--bump"
                aria-hidden="true"
                title={format!("{} bump", props.record.part.label())}
            >
                { props.record.part.emoji() }
            </div>
            <div class="entry-body">
                <div class="entry-card">
                    <div class="entry-versions">
                        <span class="entry-version entry-version--old">
                            { &props.record.old_version }
                        </span>
                        <span class="entry-arrow" aria-hidden="true">{ "→" }</span>
                        <span class="entry-version entry-version--new">
                            { &props.record.new_version }
                        </span>
                    </div>
                    <div class="entry-meta">
                        <PartBadge part={props.record.part.clone()} />
                    </div>
                </div>
            </div>
        </article>
    }
}
