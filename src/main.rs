use std::{fs::File, io::Read};

enum Identifier
{
    Import,
    Fn,
    EqualsSign,
    Quote,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftCurlyBrace,
    RightCurlyBrace,
    Identifier(String)
}

impl std::fmt::Debug for Identifier
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        match self
        {
            Self::Import => write!(f, "!Import"),
            Self::Fn => write!(f, "!Fn"),
            Self::EqualsSign => write!(f, "!EqualsSign"),
            Self::Identifier(s) => write!(f, "|{s}|"),
            Identifier::Quote => write!(f, "!\""),
            Identifier::LeftParen => write!(f, "!("),
            Identifier::RightParen => write!(f, "!)"),
            Identifier::LeftBrace => write!(f, "!["),
            Identifier::RightBrace => write!(f, "!]"),
            Identifier::LeftCurlyBrace => write!(f, "!{{"),
            Identifier::RightCurlyBrace => write!(f, "!}}")
        }
    }
}

impl Identifier
{
    fn parse(data: &[&str]) -> Vec<Identifier>
    {
        let mut output: Vec<Identifier> = Vec::new();

        for s in data.iter()
        {
            match *s
            {
                "import" => output.push(Identifier::Import),
                "fn" => output.push(Identifier::Fn),
                "=" => output.push(Identifier::EqualsSign),
                "\"" => output.push(Identifier::Quote),
                "(" => output.push(Identifier::LeftParen),
                ")" => output.push(Identifier::RightParen),
                "[" => output.push(Identifier::LeftBrace),
                "]" => output.push(Identifier::RightBrace),
                "{" => output.push(Identifier::LeftCurlyBrace),
                "}" => output.push(Identifier::RightCurlyBrace),
                _ => output.push(Identifier::Identifier(String::from(*s)))
            }
        }

        output
    }
}

fn trim_whitespace(s: &str) -> String
{
    // second attempt: only allocate a string
    let mut result = String::with_capacity(s.len());
    s.split_whitespace().for_each(|w| {
        if !result.is_empty()
        {
            result.push(' ');
        }
        result.push_str(w);
    });
    result
}

fn main()
{
    let mut file_data: String = String::new();
    File::open("main.osm").unwrap().read_to_string(&mut file_data).unwrap();

    file_data = file_data
        .replace('\n', " ")
        .replace('(', " ( ")
        .replace(')', " ) ")
        .replace('[', " [ ")
        .replace(']', " ] ")
        .replace('{', " { ")
        .replace('}', " } ");

    let mut file_strings: Vec<String> = trim_whitespace(&file_data)
        .split(' ')
        .collect::<Vec<&str>>()
        .iter()
        .map(|s| <&str as Into<String>>::into(s))
        .collect();

    file_strings.retain(|s| !s.is_empty());

    let file_identifiers =
        Identifier::parse(&file_strings.iter().map(AsRef::as_ref).collect::<Vec<&str>>());

    file_identifiers.iter().for_each(|i| {
        println!("Identifier: {i:?}");
    });

    println!("Hello, world!");
}
