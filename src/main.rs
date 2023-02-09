use std::{fs::File, io::Read};

use osmium::{tokenizer::Token, lexer::Lexeme};

fn densify(s: String) -> String
{
    s.replace(['\n', ' '], "")
}

// fn find_location_in_file

fn main()
{
    let mut raw_file: String = String::new();
    File::open("main.osm").unwrap().read_to_string(&mut raw_file).unwrap();

    let tokens: Vec<Token> = match Token::parse(&raw_file)
    {
        Ok(tokens) => tokens,
        Err((e, s)) => panic!("Compile Error: {e} @ {s}")
    };

    tokens.iter().for_each(|t| println!("Token: {t:?}"));

    // let lexemes: Vec<Lexeme> = Lexeme::lex(tokens);



}
