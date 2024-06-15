use lexer::lex;

mod grammar;
mod lexer;
mod unmarshal;
mod parse;
mod util;
//use crate::lexer;
fn main() {
    let res = lex(r#"{"hello": 12345}"#);

    match res {
        Ok(token) =>{
            print!("{:?}", token);
        }

        Err(e) => panic!("{:?}", e)
    }
}
