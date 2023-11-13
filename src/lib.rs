mod module_collector;
mod utils;

use module_collector::{ExportModule, ImportModule, ModuleCollector, ModuleType};
use swc_core::common::Span;
use swc_core::plugin::{plugin_transform, proxies::TransformPluginProgramMetadata};
use swc_core::{
    atoms::js_word,
    common::DUMMY_SP,
    ecma::{
        ast::*,
        visit::{as_folder, noop_visit_mut_type, FoldWith, VisitMut, VisitMutWith},
    },
    plugin::metadata::TransformPluginMetadataContextKind,
};
use utils::{
    call_expr, decl_var_and_assign_stmt, fn_arg, ident, ident_expr, obj_member_expr, str_lit_expr,
};

const GLOBAL: &str = "global";
const MODULE: &str = "__modules";
const MODULE_IMPORT_METHOD_NAME: &str = "import";
const MODULE_EXPORT_METHOD_NAME: &str = "export";

pub struct ReactNativeEsbuildModule {
    module_name: String,
}

impl ReactNativeEsbuildModule {
    fn default(filename: String) -> Self {
        ReactNativeEsbuildModule {
            module_name: filename,
        }
    }
}

impl ReactNativeEsbuildModule {
    fn get_custom_import_expr(&mut self, module_name: String) -> Expr {
        call_expr(
            obj_member_expr(
                obj_member_expr(ident_expr(js_word!(GLOBAL)), ident(js_word!(MODULE))),
                Ident::new(js_word!(MODULE_IMPORT_METHOD_NAME), DUMMY_SP),
            ),
            vec![fn_arg(str_lit_expr(module_name))],
        )
    }

    fn get_custom_export_expr(&mut self, export_expr: Expr) -> Expr {
        call_expr(
            obj_member_expr(
                obj_member_expr(ident_expr(js_word!(GLOBAL)), ident(js_word!(MODULE))),
                Ident::new(js_word!(MODULE_EXPORT_METHOD_NAME), DUMMY_SP),
            ),
            vec![
                fn_arg(str_lit_expr(self.module_name.to_owned())),
                fn_arg(export_expr),
            ],
        )
    }

    fn default_import_stmt(&mut self, module_name: String, span: Span, ident: Ident) -> Stmt {
        decl_var_and_assign_stmt(
            ident,
            span,
            obj_member_expr(
                self.get_custom_import_expr(module_name),
                Ident::new("default".into(), DUMMY_SP),
            ),
        )
    }

    fn named_import_stmt(&mut self, module_name: String, span: Span, ident: Ident) -> Stmt {
        decl_var_and_assign_stmt(
            ident.clone(),
            span,
            obj_member_expr(
                self.get_custom_import_expr(module_name),
                Ident::new(ident.sym, DUMMY_SP),
            ),
        )
    }

    fn namespace_import_stmt(&mut self, module_name: String, span: Span, ident: Ident) -> Stmt {
        decl_var_and_assign_stmt(
            ident.clone(),
            span,
            self.get_custom_import_expr(module_name),
        )
    }

    fn get_exports_obj_expr(&mut self, exports: Vec<ExportModule>) -> Expr {
        let mut export_props = Vec::new();
        exports.into_iter().for_each(
            |ExportModule {
                 ident,
                 as_ident,
                 module_type,
             }| {
                if let Some(prop_ident) = as_ident.or(Some(ident.clone())) {
                    export_props.push(match module_type {
                        ModuleType::Default => {
                            PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                                key: PropName::Str(Str {
                                    span: DUMMY_SP,
                                    value: js_word!("default"),
                                    raw: None,
                                }),
                                value: Box::new(Expr::Ident(ident)),
                            })))
                        }
                        ModuleType::Named => {
                            PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                                key: PropName::Str(Str {
                                    span: DUMMY_SP,
                                    value: prop_ident.sym,
                                    raw: None,
                                }),
                                value: Box::new(Expr::Ident(ident)),
                            })))
                        }
                        ModuleType::NamespaceOrAll => PropOrSpread::Spread(SpreadElement {
                            dot3_token: DUMMY_SP,
                            expr: Box::new(Expr::Ident(ident)),
                        }),
                    });
                }
            },
        );

        Expr::Object(ObjectLit {
            span: DUMMY_SP,
            props: export_props,
        })
    }

    fn get_custom_exports_stmt(&mut self, exports: Vec<ExportModule>) -> Stmt {
        let exports_obj = self.get_exports_obj_expr(exports);
        Stmt::Expr(ExprStmt {
            span: DUMMY_SP,
            expr: Box::new(self.get_custom_export_expr(exports_obj)),
        })
    }
}

impl VisitMut for ReactNativeEsbuildModule {
    noop_visit_mut_type!();

    fn visit_mut_module(&mut self, module: &mut Module) {
        let mut collector = ModuleCollector::default();
        module.visit_mut_with(&mut collector);

        let ModuleCollector { imports, exports } = collector;

        // Imports
        imports.into_iter().enumerate().for_each(
            |(
                index,
                ImportModule {
                    span,
                    ident,
                    module_src,
                    module_type,
                },
            )| match module_type {
                ModuleType::Default => {
                    module.body.insert(
                        index,
                        self.default_import_stmt(module_src, span, ident).into(),
                    );
                }
                ModuleType::Named => {
                    module.body.insert(
                        index,
                        self.named_import_stmt(module_src, span, ident).into(),
                    );
                }
                ModuleType::NamespaceOrAll => {
                    module.body.insert(
                        index,
                        self.namespace_import_stmt(module_src, span, ident).into(),
                    );
                }
            },
        );

        // Exports
        if exports.len() > 0 {
            module
                .body
                .push(self.get_custom_exports_stmt(exports).into());
        }
    }
}

#[plugin_transform]
pub fn react_native_esbuild_module_plugin(
    program: Program,
    metadata: TransformPluginProgramMetadata,
) -> Program {
    let filename = metadata
        .get_context(&TransformPluginMetadataContextKind::Filename)
        .unwrap_or_default();
    program.fold_with(&mut as_folder(ReactNativeEsbuildModule::default(filename)))
}

#[cfg(test)]
#[path = "./tests/esm_import.rs"]
mod esm_import;

#[cfg(test)]
#[path = "./tests/esm_export.rs"]
mod esm_export;
