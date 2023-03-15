use swc_common::{
    self,
    errors::{ColorConfig, Handler},
    sync::Lrc,
    SourceMap,
    Span
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

pub fn get_describe_spans(module: &swc_ecma_ast::Module) -> Vec<Span> {
    let mut spans = Vec::<Span>::new();
    for item in &module.body {
        let span = get_describe_span(item);
        if span.is_some() {
            spans.push(span.unwrap());
        }
    }
    return spans;
}

fn get_describe_span(module_item: &ModuleItem) -> Option<Span> {
    match module_item {
        ModuleItem::Stmt(stmt) => {
            match stmt {
                Stmt::Expr(expr_stmt) => {
                    let e = expr_stmt.expr.clone();
                    match *e {
                        swc_ecma_ast::Expr::Call(call_expr) => {
                            let callee = call_expr.callee.clone();
                            match callee {
                                Callee::Expr(expr) => {
                                    match *expr {
                                        swc_ecma_ast::Expr::Ident(ident) => {
                                            if ident.to_string().contains("describe") {
                                                return Some(expr_stmt.span);
                                            }
                                            None
                                        },
                                        _ => None,
                                    }
                                },
                                _ => None,
                            }
                        },
                        _ => None,
                    }
                },
                _ => None,
            }
        },
        _ => None,
    }
}

pub fn get_text_from_span(span: Span, path: &std::path::Path) -> Result<String, Box<dyn std::error::Error>> {
    let cm: Lrc<SourceMap> = Default::default();
    let fm = cm.load_file(path)?;
    let text = fm.src.get(span.lo.0 as usize..span.hi.0 as usize).unwrap();
    Ok(text.to_string())
}
