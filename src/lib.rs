use swc_core::plugin::{plugin_transform, proxies::TransformPluginProgramMetadata};
use swc_core::{
    atoms::{js_word, Atom},
    common::DUMMY_SP,
    ecma::{
        ast::*,
        visit::{as_folder, FoldWith, VisitMut},
    },
};
use tracing::debug;

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
    fn global_module_from_default_import(
        &mut self,
        default_spec: ImportDefaultSpecifier,
        module_name: &String,
    ) -> ModuleItem {
        ModuleItem::Stmt(Stmt::Decl(Decl::Var(Box::new(VarDecl {
            span: DUMMY_SP,
            kind: VarDeclKind::Var,
            declare: false,
            decls: vec![VarDeclarator {
                span: default_spec.span,
                name: Pat::Ident(BindingIdent {
                    id: default_spec.local,
                    type_ann: None,
                }),
                init: Some(Box::new(
                    self.get_custom_module_default_import_expr(module_name),
                )),
                definite: false,
            }],
        }))))
    }

    fn global_module_from_named_import(
        &mut self,
        named_spec: ImportNamedSpecifier,
        module_name: &String,
    ) -> ModuleItem {
        let local = named_spec.local.to_owned();
        let member_sym = named_spec.local.sym;

        ModuleItem::Stmt(Stmt::Decl(Decl::Var(Box::new(VarDecl {
            span: DUMMY_SP,
            kind: VarDeclKind::Var,
            declare: false,
            decls: vec![VarDeclarator {
                span: named_spec.span,
                name: Pat::Ident(BindingIdent {
                    id: local,
                    type_ann: None,
                }),
                init: Some(Box::new(
                    self.get_custom_module_named_import_expr(module_name, member_sym),
                )),
                definite: false,
            }],
        }))))
    }

    fn get_custom_module_expr(&mut self, module_name: &String) -> Expr {
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
                    value: module_name.to_owned().into(),
                }))),
                spread: None,
            }],
            type_args: None,
        })
    }

    fn get_custom_module_default_import_expr(&mut self, module_name: &String) -> Expr {
        Expr::Member(MemberExpr {
            span: DUMMY_SP,
            obj: Box::new(self.get_custom_module_expr(module_name)),
            prop: MemberProp::Ident(Ident {
                span: DUMMY_SP,
                sym: Atom::new("default"),
                optional: false,
            }),
        })
    }

    fn get_custom_module_named_import_expr(&mut self, module_name: &String, member: Atom) -> Expr {
        Expr::Member(MemberExpr {
            span: DUMMY_SP,
            obj: Box::new(self.get_custom_module_expr(module_name)),
            prop: MemberProp::Ident(Ident {
                span: DUMMY_SP,
                sym: member,
                optional: false,
            }),
        })
    }
}

impl VisitMut for ReactNativeEsbuildModule {
    fn visit_mut_module(&mut self, module: &mut Module) {
        let mut body = Vec::new();
        for module_item in &module.body {
            let is_import = match module_item {
                ModuleItem::ModuleDecl(ModuleDecl::Import(import_decl)) => {
                    let module_name = &import_decl.src.value.to_string();
                    import_decl
                        .specifiers
                        .to_owned()
                        .into_iter()
                        .for_each(|import_spec| match import_spec {
                            ImportSpecifier::Named(named_spec) => {
                                debug!(
                                    "named import: {:#?}.{:#?}",
                                    module_name, named_spec.local.sym
                                );
                                body.push(
                                    self.global_module_from_named_import(named_spec, module_name),
                                );
                            }
                            ImportSpecifier::Default(default_spec) => {
                                debug!("default import: {:#?}", module_name);
                                body.push(
                                    self.global_module_from_default_import(
                                        default_spec,
                                        module_name,
                                    ),
                                );
                            }
                            _ => {}
                        });
                    true
                }
                _ => false,
            };
            if !is_import {
                body.push(module_item.to_owned());
            }
        }
        module.body = body;
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
