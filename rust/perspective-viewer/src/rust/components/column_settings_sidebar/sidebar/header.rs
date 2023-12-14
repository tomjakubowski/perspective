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

use std::rc::Rc;

use yew::{function_component, html, Callback, Html, Properties};

use super::ColumnSettingsTab;
use crate::components::editable_header::EditableHeader;
use crate::components::type_icon::{TypeIcon, TypeIconType};
use crate::components::viewer::ColumnLocator;
use crate::config::Type;
use crate::session::Session;

#[derive(PartialEq, Properties, Clone)]
pub struct ColumnSettingsHeaderProps {
    pub maybe_ty: Option<Type>,
    pub initial_value: Option<String>,
    pub on_change: Callback<(Option<String>, bool)>,
    pub selected_column: ColumnLocator,
    pub selected_tab: ColumnSettingsTab,
    pub placeholder: Rc<String>,
    pub session: Session,
}

#[function_component(ColumnSettingsHeader)]
pub fn column_settings_header(p: &ColumnSettingsHeaderProps) -> Html {
    // let on_submit = yew::use_callback(p.clone(), move |new_name: Option<String>,
    // p| {     if let ColumnLocator::Expr(Some(column_name)) |
    // ColumnLocator::Plain(column_name) =         p.selected_column.clone()
    //     {
    //         // rename expr
    //         clone!(p, new_name);
    //         ApiFuture::spawn(async move {
    //             let update = p
    //                 .session
    //                 .create_rename_expression_update(column_name,
    // new_name.clone())                 .await;
    //             // p.presentation.set_open_column_settings(new_name);
    //             p.presentation
    //                 .set_open_column_settings(Some(OpenColumnSettings {
    //                     locator: Some(ColumnLocator::Expr(new_name.clone())),
    //                     tab: Some(p.selected_tab),
    //                 }));
    //             p.update_and_render(update).await?;
    //             Ok(())
    //         })
    //     }
    // });

    let is_expr = matches!(p.selected_column, ColumnLocator::Expr(_));
    let editable = is_expr && matches!(p.selected_tab, ColumnSettingsTab::Attributes);

    let header_icon = html! {
        <TypeIcon ty={p.maybe_ty.map(|ty| ty.into()).unwrap_or(TypeIconType::Expr)} />
    };

    html! {
        <EditableHeader
            icon={Some(header_icon)}
            on_change={p.on_change.clone()}
            {editable}
            initial_value={p.initial_value.clone()}
            placeholder={p.placeholder.clone()}
            session={p.session.clone()}
        />
    }
}
