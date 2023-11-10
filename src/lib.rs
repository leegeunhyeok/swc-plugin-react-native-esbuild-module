mod module_collector;
mod utils;

use module_collector::{ModuleCollector, ModuleMeta, ModuleType};
use swc_core::common::Span;
use swc_core::plugin::{plugin_transform, proxies::TransformPluginProgramMetadata};
use swc_core::{
    atoms::js_word,
    common::DUMMY_SP,
    ecma::{
        ast::*,
        visit::{as_folder, noop_visit_mut_type, FoldWith, VisitMut, VisitMutWith},
    },
};
use utils::{
    call_expr, decl_var_and_assign_stmt, fn_arg, ident, ident_expr, obj_member_expr, str_lit_expr,
};

const GLOBAL: &str = "global";
const MODULE: &str = "__modules";
const MODULE_GET_METHOD_NAME: &str = "get";

pub struct ReactNativeEsbuildModule;

impl Default for ReactNativeEsbuildModule {
    fn default() -> Self {
        ReactNativeEsbuildModule {}
    }
}

impl ReactNativeEsbuildModule {
    fn get_custom_module_expr(&mut self, module_name: String) -> Expr {
        call_expr(
            obj_member_expr(
                obj_member_expr(ident_expr(js_word!(GLOBAL)), ident(js_word!(MODULE))),
                Ident::new(js_word!(MODULE_GET_METHOD_NAME), DUMMY_SP),
            ),
            vec![fn_arg(str_lit_expr(module_name))],
        )
    }

    fn default_import_stmt(&mut self, module_name: String, span: Span, ident: Ident) -> Stmt {
        decl_var_and_assign_stmt(
            ident,
            span,
            obj_member_expr(
                self.get_custom_module_expr(module_name),
                Ident::new("default".into(), DUMMY_SP),
            ),
        )
    }

    fn named_import_stmt(&mut self, module_name: String, span: Span, ident: Ident) -> Stmt {
        decl_var_and_assign_stmt(
            ident.clone(),
            span,
            obj_member_expr(
                self.get_custom_module_expr(module_name),
                Ident::new(ident.sym, DUMMY_SP),
            ),
        )
    }
}

impl VisitMut for ReactNativeEsbuildModule {
    noop_visit_mut_type!();

    fn visit_mut_module(&mut self, module: &mut Module) {
        let mut collector = ModuleCollector::default();
        module.visit_mut_with(&mut collector);

        let ModuleCollector { imports } = collector;

        // Imports
        imports.into_iter().enumerate().for_each(
            |(
                index,
                ModuleMeta {
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
            },
        );
    }
}

#[plugin_transform]
pub fn react_native_esbuild_module_plugin(
    program: Program,
    _metadata: TransformPluginProgramMetadata,
) -> Program {
    program.fold_with(&mut as_folder(ReactNativeEsbuildModule::default()))
}

#[cfg(test)]
#[path = "./tests/defaults.rs"]
mod defaults;
