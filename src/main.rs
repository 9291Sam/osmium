use std::{fs::File, io::Read};

use osmium::{CompileError, tokenizer::Token};

fn main()
{
    let mut raw_file: String = String::new();
    File::open("main.osm").unwrap().read_to_string(&mut raw_file).unwrap();
    raw_file = raw_file.replace('\n', " ");
    raw_file = raw_file.replace('(', " ( ");
    raw_file = raw_file.replace(')', " ) ");
    raw_file = raw_file.replace('[', " [ ");
    raw_file = raw_file.replace(']', " ] ");
    raw_file = raw_file.replace('{', " { ");
    raw_file = raw_file.replace('}', " } ");

    let tokens: Vec<Token> = match Token::parse(&raw_file)
    {
        Ok(tokens) => tokens,
        Err(e) => panic!("Compile Error: {:?}", CompileError::from(e))
    };

    tokens.iter().for_each(|t| println!("Token: {t:?}"));
}
