mod module_collector;
use module_collector::{ModuleCollector, ModuleMeta, ModuleType};
use swc_core::common::Span;
use swc_core::plugin::{plugin_transform, proxies::TransformPluginProgramMetadata};
use swc_core::{
    atoms::{js_word, Atom},
    common::{util::take::Take, DUMMY_SP},
    ecma::{
        ast::*,
        visit::{as_folder, noop_visit_mut_type, FoldWith, VisitMut, VisitMutWith},
    },
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
        Expr::Call(CallExpr {
            span: DUMMY_SP,
            callee: Callee::Expr(Box::new(Expr::Member(MemberExpr {
                span: DUMMY_SP,
                obj: Box::new(Expr::Member(MemberExpr {
                    span: DUMMY_SP,
                    obj: Box::new(Expr::Ident(Ident::new(js_word!(GLOBAL), DUMMY_SP))),
                    prop: MemberProp::Ident(Ident::new(js_word!(MODULE), DUMMY_SP)),
                })),
                prop: MemberProp::Ident(Ident::new(js_word!(MODULE_GET_METHOD_NAME), DUMMY_SP)),
            }))),
            args: vec![ExprOrSpread {
                expr: Box::new(Expr::Lit(Lit::Str(Str {
                    span: DUMMY_SP,
                    raw: None,
                    value: Atom::new(module_name),
                }))),
                spread: None,
            }],
            type_args: None,
        })
    }

    fn default_import_stmt(&mut self, module_name: String, span: Span, ident: Ident) -> Stmt {
        Stmt::Decl(Decl::Var(Box::new(VarDecl {
            span: DUMMY_SP,
            kind: VarDeclKind::Var,
            declare: false,
            decls: vec![VarDeclarator {
                span,
                name: Pat::Ident(BindingIdent {
                    id: ident.clone(),
                    type_ann: None,
                }),
                init: Some(Box::new(Expr::Member(MemberExpr {
                    span: DUMMY_SP,
                    obj: Box::new(self.get_custom_module_expr(module_name)),
                    prop: MemberProp::Ident(Ident {
                        span: DUMMY_SP,
                        sym: Atom::new("default"),
                        optional: false,
                    }),
                }))),
                definite: false,
            }],
        })))
    }

    fn named_import_stmt(&mut self, module_name: String, span: Span, ident: Ident) -> Stmt {
        Stmt::Decl(Decl::Var(Box::new(VarDecl {
            span: DUMMY_SP,
            kind: VarDeclKind::Var,
            declare: false,
            decls: vec![VarDeclarator {
                span,
                name: Pat::Ident(BindingIdent {
                    id: ident.clone(),
                    type_ann: None,
                }),
                init: Some(Box::new(Expr::Member(MemberExpr {
                    span: DUMMY_SP,
                    obj: Box::new(self.get_custom_module_expr(module_name)),
                    prop: MemberProp::Ident(Ident {
                        span: DUMMY_SP,
                        sym: ident.sym.clone(),
                        optional: false,
                    }),
                }))),
                definite: false,
            }],
        })))
    }
}

impl VisitMut for ReactNativeEsbuildModule {
    noop_visit_mut_type!();

    fn visit_mut_module(&mut self, module: &mut Module) {
        let mut collector = ModuleCollector::default();
        module.visit_mut_with(&mut collector);

        let mut module_body = Vec::with_capacity(module.body.len());
        let ModuleCollector { imports } = collector;

        // Imports
        imports.into_iter().for_each(
            |ModuleMeta {
                 span,
                 ident,
                 module_src,
                 module_type,
             }| match module_type {
                ModuleType::Default => {
                    module_body.push(self.default_import_stmt(module_src, span, ident).into());
                }
                ModuleType::Named => {
                    module_body.push(self.named_import_stmt(module_src, span, ident).into());
                }
            },
        );

        module_body.extend(module.body.take());

        module.body = module_body;
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
