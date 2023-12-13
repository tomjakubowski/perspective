use std::rc::Rc;

use yew::{function_component, html, Callback, Html, Properties};

use crate::components::column_settings_sidebar::attributes_tab::AttributesTab;
use crate::components::column_settings_sidebar::style_tab::StyleTab;
use crate::components::containers::tablist::TabList;
use crate::components::viewer::ColumnLocator;
use crate::config::Type;
use crate::custom_events::CustomEvents;
use crate::presentation::Presentation;
use crate::renderer::Renderer;
use crate::session::Session;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum ColumnSettingsTab {
    #[default]
    Attributes,
    Style,
}

#[derive(Clone, Properties)]
pub struct ColumnSettingsTablistProps {
    pub renderer: Renderer,
    pub presentation: Presentation,
    pub session: Session,
    pub custom_events: CustomEvents,
    pub on_close: Callback<()>,
    pub selected_column: ColumnLocator,
    pub on_expr_input: Callback<Rc<String>>,
    pub on_tab_change: Callback<(usize, ColumnSettingsTab)>,
    pub selected_tab: (usize, ColumnSettingsTab),
    pub tabs: Vec<ColumnSettingsTab>,
    pub maybe_ty: Option<Type>,
    pub header_value: Option<String>,
    pub column_name: String,
    pub is_active: bool,
}

impl PartialEq for ColumnSettingsTablistProps {
    fn eq(&self, other: &Self) -> bool {
        self.selected_column == other.selected_column
            && self.column_name == other.column_name
            && self.selected_tab == other.selected_tab
            && self.tabs == other.tabs
            && self.is_active == other.is_active
            && self.header_value == other.header_value
    }
}

#[function_component(ColumnSettingsTablist)]
pub fn column_settings_tablist(p: &ColumnSettingsTablistProps) -> Html {
    let match_fn = yew::use_callback(p.clone(), move |tab, p| match tab {
        ColumnSettingsTab::Attributes => {
            html! {
                <AttributesTab
                    session={ p.session.clone() }
                    renderer={ p.renderer.clone() }
                    custom_events={ p.custom_events.clone() }
                    presentation={ p.presentation.clone() }

                    selected_column={ p.selected_column.clone() }
                    on_close={ p.on_close.clone() }
                    header_value={p.header_value.clone()}
                    on_input={p.on_expr_input.clone()}
                />
            }
        },
        ColumnSettingsTab::Style => html! {
            <StyleTab
                session={ p.session.clone() }
                renderer={ p.renderer.clone() }
                custom_events={ p.custom_events.clone() }

                column_name={ p.column_name.clone() }
                ty={ p.maybe_ty.unwrap() }
            />
        },
    });

    let selected_tab = if p.selected_tab.0 >= p.tabs.len() {
        0
    } else {
        p.selected_tab.0
    };

    html! {
        <TabList<ColumnSettingsTab>
            tabs={p.tabs.clone()}
            {match_fn}
            on_tab_change={p.on_tab_change.clone()}
            {selected_tab}
        />
    }
}
