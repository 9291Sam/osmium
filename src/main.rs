use std::{fs::File, io::Read};

use osmium::{
    lexer::Lexeme,
    tokenizer::{PinnedToken, PinnedTokenizationError, Token},
};

// (line, column)
fn find_line_and_column(raw_file: &str, str_to_find: &str) -> Option<(usize, usize)> {
    let idx_of_found: usize = raw_file.find(str_to_find)? + 1;

    let number_of_lines = raw_file[..idx_of_found].matches('\n').count();

    let column = raw_file
        .splitn(number_of_lines, '\n')
        .nth(number_of_lines - 1)
        .unwrap()
        .find(str_to_find)
        .unwrap();

    Some((number_of_lines, column))
}

fn main() {
    let mut raw_file: String = String::new();
    File::open("main.osm").unwrap().read_to_string(&mut raw_file).unwrap();

    let tokens: Vec<PinnedToken> = match Token::parse(&raw_file) {
        Ok(tokens) => tokens,
        Err(e) => panic!(
            "Tokenization Error: {e:?} @ {:?}",
            find_line_and_column(&raw_file, &raw_file.as_str()[e.location..])
        ),
    };

    tokens.iter().for_each(|t| println!("Token: {t:?}"));

    let _lexemes: Vec<Lexeme> = Lexeme::lex(tokens);
}
