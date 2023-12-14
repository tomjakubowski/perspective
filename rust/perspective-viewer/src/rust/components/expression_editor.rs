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

use wasm_bindgen::prelude::*;
use yew::prelude::*;

use super::containers::split_panel::*;
use super::form::code_editor::*;
use super::style::LocalStyle;
use crate::js::PerspectiveValidationError;
use crate::session::Session;
use crate::*;

#[derive(Debug)]
pub enum ExpressionEditorMsg {
    Reset,
    Delete,
    SetExpr(Rc<String>),
    ValidateComplete(Option<PerspectiveValidationError>),
    SaveExpr,
}

#[derive(Properties, PartialEq)]
pub struct ExpressionEditorProps {
    pub session: Session,
    pub on_save: Callback<JsValue>,
    pub on_validate: Callback<bool>,
    pub on_delete: Option<Callback<()>>,
    pub on_input: Callback<Rc<String>>,
    pub on_reset: Callback<()>,
    pub alias: Option<String>,
    pub disabled: bool,
    pub valid_alias: bool,
    pub alias_changed: bool,
}

impl ExpressionEditorProps {
    fn initial_expr(&self) -> Rc<String> {
        self.alias
            .as_ref()
            .and_then(|alias| self.session.metadata().get_expression_by_alias(alias))
            .unwrap_or_default()
            .into()
    }
}

/// Expression editor component `CodeEditor` and a button toolbar.
pub struct ExpressionEditor {
    save_enabled: bool,
    reset_enabled: bool,
    expr: Rc<String>,
    error: Option<PerspectiveValidationError>,
}

impl Component for ExpressionEditor {
    type Message = ExpressionEditorMsg;
    type Properties = ExpressionEditorProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            save_enabled: false,
            reset_enabled: false,
            error: None,
            expr: ctx.props().initial_expr(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        tracing::info!("<ExpressionEditor> update, msg = {:#?}", msg);

        match msg {
            ExpressionEditorMsg::SetExpr(val) => {
                ctx.props().on_input.emit(val.clone());
                self.reset_enabled = ctx.props().initial_expr() != val;
                self.expr = val.clone();
                clone!(ctx.props().session);
                ctx.props().on_validate.emit(true);
                ctx.link().send_future(async move {
                    match session.validate_expr(&val).await {
                        Ok(x) => ExpressionEditorMsg::ValidateComplete(x),
                        Err(err) => {
                            web_sys::console::error_1(&err);
                            ExpressionEditorMsg::ValidateComplete(None)
                        },
                    }
                });

                true
            },
            ExpressionEditorMsg::ValidateComplete(err) => {
                self.error = err;
                if self.error.is_none() {
                    let is_edited = maybe!({
                        let alias = ctx.props().alias.as_ref()?;
                        let session = &ctx.props().session;
                        let old = session.metadata().get_expression_by_alias(alias)?;
                        let is_edited = *self.expr != old;
                        session
                            .metadata_mut()
                            .set_edit_by_alias(alias, self.expr.to_string());
                        Some(is_edited)
                    });

                    self.save_enabled = is_edited.unwrap_or(true) && ctx.props().valid_alias;
                } else {
                    self.save_enabled = false;
                }

                ctx.props().on_validate.emit(false);
                true
            },
            ExpressionEditorMsg::Reset => {
                self.reset_enabled = false;
                self.save_enabled = false;
                self.expr = ctx.props().initial_expr();
                ctx.props().on_reset.emit(());
                ctx.link()
                    .send_message(ExpressionEditorMsg::SetExpr(self.expr.clone()));
                true
            },
            ExpressionEditorMsg::SaveExpr => {
                if self.save_enabled {
                    let expr = self.expr.to_owned();
                    ctx.props().on_save.emit(JsValue::from(&*expr));
                    self.reset_enabled = false;
                    self.save_enabled = false;
                    true
                } else {
                    tracing::error!("Tried to save expression when save is disabled!");
                    false
                }
            },
            ExpressionEditorMsg::Delete => {
                if let Some(on_delete) = &ctx.props().on_delete {
                    on_delete.emit(());
                }

                false
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let reset = ctx.link().callback(|_| ExpressionEditorMsg::Reset);
        let delete = ctx.link().callback(|_| ExpressionEditorMsg::Delete);
        let save = ctx.link().callback(|_| ExpressionEditorMsg::SaveExpr);
        let oninput = ctx.link().callback(ExpressionEditorMsg::SetExpr);
        let onsave = ctx.link().callback(|()| ExpressionEditorMsg::SaveExpr);
        let delete_hidden = ctx
            .props()
            .alias
            .as_ref()
            .map(|alias| ctx.props().session.is_column_expression_in_use(alias))
            .unwrap_or_default()
            || ctx.props().alias.is_none();

        let disabled_class = ctx.props().disabled.then_some("disabled");
        clone!(ctx.props().disabled);

        html_template! {
            <LocalStyle href={ css!("expression-editor") } />
            <SplitPanel orientation={ Orientation::Vertical }>
                <>
                    <label class="item_title">{ "Expression" }</label>
                    <div id="editor-container" class={ disabled_class }>
                        <CodeEditor
                            expr={ &self.expr }
                            error={ self.error.clone().map(|x| x.into()) }
                            { disabled }
                            { oninput }
                            { onsave }/>

                        <div id="psp-expression-editor-meta">
                            <div class="error">
                                {&self.error.clone().map(|e| e.error_message).unwrap_or_default()}
                            </div>
                        </div>
                    </div>
                </>
                <></>
            </SplitPanel>

            // TODO: This should be its own component.
            if !delete_hidden {
                <div id="danger-zone">
                    <button
                        id="psp-expression-editor-button-delete"
                        class="psp-expression-editor__button"
                        onmousedown={ delete }>
                        { "Delete Column" }
                    </button>
                </div>
            }

            // TODO: This should be its own component.
            <div id="save-settings">
                <button
                    id="psp-expression-editor-button-reset"
                    class="psp-expression-editor__button"
                    onmousedown={ reset }
                    disabled={ !self.reset_enabled }>
                    { "Reset" }
                </button>

                <button
                    id="psp-expression-editor-button-save"
                    class="psp-expression-editor__button"
                    onmousedown={ save }
                    disabled={ !self.save_enabled }>
                    { "Save" }
                </button>
            </div>
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        let alias_changed = ctx.props().alias_changed;
        tracing::info!("<ExpressionEditor>::changed {}", alias_changed);
        if ctx.props().alias != old_props.alias {
            // Selected column has changed
            self.expr = ctx.props().initial_expr();
        }
        if alias_changed && ctx.props().valid_alias {
            self.save_enabled = true;
        }
        // TODO: handle disable save when alias goes back to original value

        true
    }
}
