use swc_core::{
    common::Span,
    ecma::{
        ast::*,
        visit::{VisitMut, VisitMutWith},
    },
};
use tracing::debug;

pub type Import = Vec<ModuleMeta>;

#[derive(Debug)]
pub enum ModuleType {
    Default,
    Named,
}

#[derive(Debug)]
pub struct ModuleMeta {
    pub span: Span,
    pub ident: Ident,
    pub module_src: String,
    pub module_type: ModuleType,
}

pub struct ModuleCollector {
    pub imports: Import,
}

impl ModuleCollector {
    // TODO
}

impl Default for ModuleCollector {
    fn default() -> Self {
        ModuleCollector {
            imports: Vec::new(),
        }
    }
}

impl VisitMut for ModuleCollector {
    fn visit_mut_module(&mut self, module: &mut Module) {
        let mut module_body = Vec::with_capacity(module.body.len());
        for module_item in module.body.drain(..) {
            match module_item {
                ModuleItem::Stmt(stmt) => module_body.push(stmt.into()),
                ModuleItem::ModuleDecl(mut module_decl) => match module_decl {
                    ModuleDecl::Import(_) => {
                        module_decl.visit_mut_with(self);
                    }
                    _ => module_body.push(module_decl.into()),
                },
            };
        }
        module.body = module_body;
    }

    fn visit_mut_import_decl(&mut self, import_decl: &mut ImportDecl) {
        import_decl
            .specifiers
            .to_owned()
            .into_iter()
            .for_each(|import_spec| match import_spec {
                ImportSpecifier::Default(ImportDefaultSpecifier { span, local }) => {
                    debug!("default import: {:#?}", local.sym);
                    self.imports.push(ModuleMeta {
                        span,
                        ident: local,
                        module_src: import_decl.src.value.to_string(),
                        module_type: ModuleType::Default,
                    });
                }
                ImportSpecifier::Named(ImportNamedSpecifier { span, local, .. }) => {
                    debug!("named import: {:#?}", local.sym);
                    self.imports.push(ModuleMeta {
                        span,
                        ident: local,
                        module_src: import_decl.src.value.to_string(),
                        module_type: ModuleType::Named,
                    });
                }
                _ => (),
            });
    }
}
