mod gen;
mod parse;
mod tokenize;

use gen::gen;
use parse::parse;
use tokenize::tokenize;

pub fn get_table(input: String) -> Result<String, String> {
    match tokenize(&input) {
        Ok(tokens) => match parse(tokens) {
            Ok(ast) => {
                Ok(gen(&ast))
            }
            Err(err) => Err(err),
        },
        Err(err) => Err(err),
    }
}
