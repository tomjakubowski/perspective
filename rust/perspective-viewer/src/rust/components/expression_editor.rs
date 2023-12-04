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
    pub alias: Option<String>,
    pub disabled: bool,
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
    edit_enabled: bool,
    expr: Rc<String>,
    error: Option<PerspectiveValidationError>,
}

impl Component for ExpressionEditor {
    type Message = ExpressionEditorMsg;
    type Properties = ExpressionEditorProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            save_enabled: false,
            edit_enabled: false,
            error: None,
            expr: ctx.props().initial_expr(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ExpressionEditorMsg::SetExpr(val) => {
                ctx.props().on_input.emit(val.clone());
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

                    self.edit_enabled = is_edited.unwrap_or_default();
                    self.save_enabled = is_edited.unwrap_or(true);
                } else {
                    self.save_enabled = false;
                }

                ctx.props().on_validate.emit(false);
                true
            },
            ExpressionEditorMsg::Reset => {
                self.edit_enabled = false;
                self.save_enabled = false;
                maybe!({
                    let alias = ctx.props().alias.as_ref()?;
                    let session = &ctx.props().session;
                    let old = session.metadata().get_expression_by_alias(alias)?;
                    self.expr = old.into();
                    Some(())
                })
                .unwrap_or_default();

                true
            },
            ExpressionEditorMsg::SaveExpr => {
                if self.save_enabled {
                    let expr = self.expr.to_owned();
                    ctx.props().on_save.emit(JsValue::from(&*expr));
                    self.edit_enabled = false;
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
                    <div>
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

                                <div id="psp-expression-editor-actions">
                                    <button
                                        id="psp-expression-editor-button-delete"
                                        class="psp-expression-editor__button"
                                        style = {delete_hidden.then_some("visibility: hidden")}
                                        onmousedown={ delete }>
                                        { "Delete" }
                                    </button>

                                    <button
                                        id="psp-expression-editor-button-reset"
                                        class="psp-expression-editor__button"
                                        style = {ctx.props().alias.is_none().then_some("visibility: hidden")}
                                        onmousedown={ reset }
                                        disabled={ !self.edit_enabled }>
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
                            </div>
                        </div>
                    </div>
                </>
                <></>
            </SplitPanel>
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        if ctx.props().alias != old_props.alias {
            self.expr = ctx.props().initial_expr();
        }
        true
    }
}
