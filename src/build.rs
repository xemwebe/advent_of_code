use cfgrammar::yacc::YaccKind;
use lrlex::CTLexerBuilder;

fn main() {
    CTLexerBuilder::new()
        .lrpar_config(|ctp| {
            ctp.yacckind(YaccKind::Grmtools)
                .grammar_in_src_dir("year2022/calc.y")
                .unwrap()
        })
        .lexer_in_src_dir("year2022/calc.l")
        .unwrap()
        .build()
        .unwrap();
    Ok(())
}