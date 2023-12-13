use std::rc::Rc;

use yew::{function_component, html, Callback, Html, Properties};

use super::ColumnSettingsTab;
use crate::components::editable_header::EditableHeader;
use crate::components::type_icon::{TypeIcon, TypeIconType};
use crate::components::viewer::ColumnLocator;
use crate::config::Type;
use crate::model::*;
use crate::presentation::{OpenColumnSettings, Presentation};
use crate::renderer::Renderer;
use crate::session::Session;
use crate::utils::ApiFuture;
use crate::{clone, derive_model};

#[derive(PartialEq, Properties, Clone)]
pub struct ColumnSettingsHeaderProps {
    pub maybe_ty: Option<Type>,
    pub header_value: Option<String>,
    pub on_change: Callback<Option<String>>,
    pub selected_column: ColumnLocator,
    pub selected_tab: ColumnSettingsTab,
    pub session: Session,
    pub renderer: Renderer,
    pub placeholder: Rc<String>,
    pub presentation: Presentation,
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
    let editable = is_expr && matches!(p.selected_tab, ColumnSettingsTab::Attributes);

    let header_icon = html! {
        <TypeIcon ty={p.maybe_ty.map(|ty| ty.into()).unwrap_or(TypeIconType::Expr)} />
    };

    html! {
        <EditableHeader
            icon={Some(header_icon)}
            on_change={p.on_change.clone()}
            {on_submit}
            {editable}
            value={p.header_value.clone()}
            placeholder={p.placeholder.clone()}
        />
    }
}
