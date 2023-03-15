use swc_common::{
    self,
    errors::{ColorConfig, Handler},
    sync::Lrc,
    SourceMap
};
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax};
use swc_ecma_ast::{self, EsVersion, ModuleItem, Stmt, Callee, Module};

pub fn get_ast_from_file(path: &std::path::Path) -> Result<Module, Box<dyn std::error::Error>> {
    let cm: Lrc<SourceMap> = Default::default();
    let fm = cm.load_file(path)?;
    println!("{:?}", fm);
    
    let handler = Handler::with_tty_emitter(
        ColorConfig::Auto,
        true,
        false,
        Some(cm.clone()),
    );
    let lexer = Lexer::new(
        Syntax::default(),
        EsVersion::Es2019,
        StringInput::from(&*fm),
        None
    );
    let mut parser = Parser::new_from(lexer);
    let module = parser.parse_module().map_err(|e| e.into_diagnostic(&handler).emit()).expect("failed to parse module");
    Ok(module)
}

pub fn get_text_from_module(module: &swc_ecma_ast::Module) -> String {
    let mut text = String::new();
    for item in &module.body {
        match_describe(item);
        text.push_str(&format!("{:?}", item));
    }
    text
}

fn match_describe(module_item: &ModuleItem) {
    match module_item {
        ModuleItem::Stmt(stmt) => {
            match stmt {
                Stmt::Expr(expr) => {
                    let e = expr.expr.clone();
                    match *e {
                        swc_ecma_ast::Expr::Call(call_expr) => {
                            let callee = call_expr.callee.clone();
                            match callee {
                                Callee::Expr(expr) => {
                                    match *expr {
                                        swc_ecma_ast::Expr::Ident(ident) => {
                                            println!("Ident: {:?}", ident.sym);
                                        },
                                        _ => (),
                                    }
                                },
                                _ => (),
                            }
                        },
                        _ => (),
                    }
                },
                _ => (),
            }
        },
        _ => (),
    }
}
