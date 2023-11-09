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

use std::fmt::Display;
use std::rc::Rc;

use yew::{function_component, html, Callback, Html, Properties};

use crate::components::column_settings_sidebar::attributes_tab::AttributesTab;
use crate::components::column_settings_sidebar::style_tab::StyleTab;
use crate::components::containers::sidebar::Sidebar;
use crate::components::containers::tablist::{Tab, TabList};
use crate::components::editable_header::EditableHeader;
use crate::components::style::LocalStyle;
use crate::components::type_icon::{TypeIcon, TypeIconType};
use crate::components::viewer::ColumnLocator;
use crate::config::Type;
use crate::custom_events::CustomEvents;
use crate::model::*;
use crate::presentation::{OpenColumnSettings, Presentation};
use crate::renderer::Renderer;
use crate::session::Session;
use crate::utils::ApiFuture;
use crate::{clone, css, derive_model, html_template};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum ColumnSettingsTab {
    #[default]
    Attributes,
    Style,
}

impl Display for ColumnSettingsTab {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{self:?}"))
    }
}

impl Tab for ColumnSettingsTab {}

#[derive(Clone, Properties)]
pub struct ColumnSettingsProps {
    pub selected_column: ColumnLocator,
    pub on_close: Callback<()>,
    pub session: Session,
    pub renderer: Renderer,
    pub presentation: Presentation,
    pub custom_events: CustomEvents,
    pub width_override: Option<i32>,
    pub is_active: bool,
}

derive_model!(CustomEvents, Session, Renderer, Presentation for ColumnSettingsProps);

impl PartialEq for ColumnSettingsProps {
    fn eq(&self, other: &Self) -> bool {
        self.selected_column == other.selected_column && self.is_active == other.is_active
    }
}

#[function_component]
pub fn ColumnSettingsSidebar(p: &ColumnSettingsProps) -> Html {
    let column_name = p.selected_column.name_or_default(&p.session);
    let header_value = yew::use_state_eq(|| Some(column_name.clone()));
    {
        // on p.selected_column change...
        clone!(header_value, p.session);
        yew::use_effect_with(p.selected_column.clone(), move |selected_column| {
            header_value.set(Some(selected_column.name_or_default(&session)));
        });
    }
    let on_change_header_value = yew::use_callback(header_value.clone(), |s, header_value| {
        header_value.set(s);
    });

    let expr_contents = {
        let expr_contents = p
            .session
            .metadata()
            .get_expression_by_alias(&column_name)
            .unwrap_or_default();
        yew::use_state(move || Rc::new(expr_contents))
    };
    let on_expr_input = yew::use_callback(expr_contents.clone(), |val, expr_contents| {
        expr_contents.set(val)
    });

    let maybe_ty = p.session.metadata().get_column_view_type(&column_name);

    // --- tabs ---
    let (config, attrs) = (p.get_plugin_config(), p.get_plugin_attrs());
    if config.is_none() || attrs.is_none() {
        tracing::warn!(
            "Could not get full plugin config!\nconfig (plugin.save()): {:?}\nplugin_attrs: {:?}",
            config,
            attrs
        );
    }

    let mut tabs = vec![];
    // TODO: This is a hack and needs to be replaced.
    let plugin = p.renderer.get_active_plugin().unwrap();
    let show_styles = maybe_ty
        .map(|ty| match &*plugin.name() {
            "Datagrid" => ty != Type::Bool,
            "X/Y Scatter" => ty == Type::String,
            _ => false,
        })
        .unwrap_or_default();

    if !matches!(p.selected_column, ColumnLocator::Expr(None)) && show_styles && config.is_some() {
        tabs.push(ColumnSettingsTab::Style);
    }

    if matches!(p.selected_column, ColumnLocator::Expr(_)) {
        tabs.push(ColumnSettingsTab::Attributes);
    }
    let selected_tab = yew::use_state(|| (0, *tabs.first().unwrap()));
    let on_tab_change = yew::use_callback(selected_tab.clone(), move |(i, tab), selected_tab| {
        selected_tab.set((i, tab));
    });

    // --- header ---
    let header_contents = html! {
        <ColumnSettingsHeader
            {maybe_ty}
            header_value={(*header_value).clone()}
            on_change={on_change_header_value.clone()}
            selected_tab={selected_tab.1}
            selected_column={p.selected_column.clone()}
            placeholder={(*expr_contents).clone()}
            session={p.session.clone()}
            renderer={p.renderer.clone()}
            presentation={p.presentation.clone()}
        />
    };

    // --- render ---
    html_template! {
        <LocalStyle href={ css!("column-settings-panel") } />
        <Sidebar
            on_close={p.on_close.clone()}
            id_prefix="column_settings"
            width_override={p.width_override}
            selected_tab={selected_tab.0}
            {header_contents}
        >
            <ColumnSettingsTablist
                renderer={p.renderer.clone()}
                presentation={p.presentation.clone()}
                session={p.session.clone()}
                custom_events={p.custom_events.clone()}

                on_close={p.on_close.clone()}
                selected_column={p.selected_column.clone()}
                {on_expr_input}

                on_tab_change={on_tab_change.clone()}
                selected_tab={*selected_tab}
                {tabs}
                {maybe_ty}
                header_value={(*header_value).clone()}
                {column_name}
                is_active={p.is_active}
            />

        </Sidebar>

    }
}

#[derive(Clone, Properties)]
pub struct ColumnSettingsTablistProps {
    renderer: Renderer,
    presentation: Presentation,
    session: Session,
    custom_events: CustomEvents,

    on_close: Callback<()>,
    selected_column: ColumnLocator,
    on_expr_input: Callback<Rc<String>>,

    on_tab_change: Callback<(usize, ColumnSettingsTab)>,
    selected_tab: (usize, ColumnSettingsTab),

    tabs: Vec<ColumnSettingsTab>,
    maybe_ty: Option<Type>,
    header_value: Option<String>,
    column_name: String,
    is_active: bool,
}
derive_model!(Renderer, Presentation, Session, CustomEvents for ColumnSettingsTablistProps);

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

#[derive(PartialEq, Properties, Clone)]
pub struct ColumnSettingsHeaderProps {
    maybe_ty: Option<Type>,
    header_value: Option<String>,
    on_change: Callback<Option<String>>,
    selected_column: ColumnLocator,
    selected_tab: ColumnSettingsTab,
    session: Session,
    renderer: Renderer,
    placeholder: Rc<String>,
    presentation: Presentation,
}
derive_model!(Presentation, Session, Renderer for ColumnSettingsHeaderProps);

#[function_component(ColumnSettingsHeader)]
pub fn column_settings_header(p: &ColumnSettingsHeaderProps) -> Html {
    let on_submit = yew::use_callback(p.clone(), move |new_name: Option<String>, p| {
        if let ColumnLocator::Expr(Some(column_name)) | ColumnLocator::Plain(column_name) =
            p.selected_column.clone()
        {
            // rename expr
            clone!(p, new_name);
            ApiFuture::spawn(async move {
                let update = p
                    .session
                    .create_rename_expression_update(column_name, new_name.clone())
                    .await;
                // p.presentation.set_open_column_settings(new_name);
                p.presentation
                    .set_open_column_settings(Some(OpenColumnSettings {
                        locator: Some(ColumnLocator::Expr(new_name.clone())),
                        tab: Some(p.selected_tab),
                    }));
                p.update_and_render(update).await?;
                Ok(())
            })
        }
    });

    let is_expr = matches!(p.selected_column, ColumnLocator::Expr(_));
    let icon_type = if is_expr {
        TypeIconType::Expr
    } else {
        p.maybe_ty.map(|t| t.into()).unwrap_or(TypeIconType::Expr)
    };
    let header_icon = html! {<TypeIcon ty={icon_type} />};

    html! {
        <EditableHeader
            icon={Some(header_icon)}
            on_change={p.on_change.clone()}
            {on_submit}
            editable={is_expr}
            value={p.header_value.clone()}
            placeholder={p.placeholder.clone()}
        />
    }
}
