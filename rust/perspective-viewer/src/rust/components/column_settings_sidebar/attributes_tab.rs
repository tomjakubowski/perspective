// ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
// ┃ ██████ ██████ ██████       █      █      █      █      █ █▄  ▀███ █       ┃
// ┃ ▄▄▄▄▄█ █▄▄▄▄▄ ▄▄▄▄▄█  ▀▀▀▀▀█▀▀▀▀▀ █ ▀▀▀▀▀█ ████████▌▐███ ███▄  ▀█ █ ▀▀▀▀▀ ┃
// ┃ █▀▀▀▀▀ █▀▀▀▀▀ █▀██▀▀ ▄▄▄▄▄ █ ▄▄▄▄▄█ ▄▄▄▄▄█ ████████▌▐███ █████▄   █ ▄▄▄▄▄ ┃
// ┃ █      ██████ █  ▀█▄       █ ██████      █      ███▌▐███ ███████▄ █       ┃
// ┣━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┫
// ┃ Copyright (c) 2017, the Perspective Authors.                              ┃
// ┃ ╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌ ┃
// ┃ This file is part of the Perspective library, distributed under the terms ┃
// ┃ of the [Apache License 2.0](https://www.apache.org/licenses/LICENSE-2.0). ┃
// ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

mod expression_editor;

use std::rc::Rc;

use expression_editor::ExprEditorAttr;
use yew::{function_component, html, Callback, Html, Properties};

use crate::components::viewer::ColumnLocator;
use crate::custom_events::CustomEvents;
use crate::presentation::Presentation;
use crate::renderer::Renderer;
use crate::session::Session;
use crate::{derive_model, html_template};

#[derive(PartialEq, Clone, Properties)]
pub struct AttributesTabProps {
    pub header_value: Option<String>,
    pub initial_header_value: Option<String>,
    pub header_valid: bool,
    pub selected_column: ColumnLocator,
    pub on_close: Callback<()>,
    pub on_reset: Callback<()>,
    pub session: Session,
    pub renderer: Renderer,
    pub presentation: Presentation,
    pub custom_events: CustomEvents,
    pub on_input: Callback<Rc<String>>,
}
derive_model!(Session, Renderer, Presentation for AttributesTabProps);

#[function_component]
pub fn AttributesTab(p: &AttributesTabProps) -> Html {
    tracing::info!(
        "<AttributesTab> header_value {:?} (initial {:?})",
        p.header_value,
        p.initial_header_value
    );
    html_template! {
        <div id="attributes-tab">
            <div class="tab-section">
                <ExprEditorAttr
                    on_close={p.on_close.clone()}
                    on_reset={p.on_reset.clone()}
                    header_value={p.header_value.clone()}
                    header_valid={p.header_valid}
                    header_changed={p.header_value != p.initial_header_value}
                    selected_column={p.selected_column.clone()}
                    session={p.session.clone()}
                    renderer={p.renderer.clone()}
                    on_input={p.on_input.clone()}
                    presentation={p.presentation.clone()}
                />
            </div>
        </div>
    }
}
