use std::{fs::File, io::Read};

use osmium::{
    lexer::Lexeme,
    tokenizer::{FileToken, Token},
};

fn main() {
    let mut raw_file: String = String::new();
    File::open("main.osm").unwrap().read_to_string(&mut raw_file).unwrap();

    let tokens: Vec<FileToken> = match Token::parse(&raw_file)
    {
        Ok(tokens) => tokens,
        Err(e) => panic!("{}", e),
    };

    tokens.iter().for_each(|t| println!("{t}"));

    let _lexemes = Lexeme::lex(tokens);
}
