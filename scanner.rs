#[derive(Debug,PartialEq)]
enum ScanError {
    UnterminatedString,
    EmptyKeyword,
    RanOff,
}

#[derive(Debug,PartialEq)]
enum TokenType {
    // parens
    OpenParenthesis, CloseParenthesis,
    OpenBracket, CloseBracket,
    OpenBrace, CloseBrace,
    // literals
    String, Number,
    // special syntax
    Quote,
    // keywords
    Keyword,
    // symbols
    Symbol,
    // we're done here
    EOF,
    // i am
    Error(ScanError),
}

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    line: usize,
    start: usize,
    length: usize,
}

impl Token {
    pub fn is_error(&self) -> bool {
        match self.token_type {
            TokenType::Error(_) => true,
            _ => false,
        }
    }

    pub fn get_token(&self, source: &Vec<char>) -> String {
        source[self.start..self.start+self.length]
            .into_iter()
            .collect()
    }
}

fn is_number(c: char) -> bool {
    c.is_ascii_digit()
        || c == '.'
}

fn is_symbol(c: char) -> bool {
    c.is_alphanumeric()
        || c == '-'
        || c == '_'
        || c == '>'
        || c == '<'
        || c == '*'
        || c == '+'
        || c == '.'
        || c == '!'
        || c == '?'
        || c == '/'
        || c == ':'
        || c == '='
}

fn advance(source: &Vec<char>, offset: &mut usize, line: &mut usize) {
    if source[*offset] == '\n' {
        *line += 1;
    }
    *offset += 1;
}

fn scan_token(source: &Vec<char>, offset: usize, line: &mut usize) -> Token {
    let mut start = offset;
    while start < source.len() - 1 && source[start].is_whitespace() {
        advance(source, &mut start, line);
    }
    if source[start] == ';' {
        while start < source.len() - 1 && source[start] != '\n' {
            advance(source, &mut start, line);
        }
        advance(source, &mut start, line); // skip the newline
    }
    let (token_type, length) = match source[start] {
        '(' => (TokenType::OpenParenthesis, 1),
        ')' => (TokenType::CloseParenthesis, 1),
        '[' => (TokenType::OpenBracket, 1),
        ']' => (TokenType::CloseBracket, 1),
        '{' => (TokenType::OpenBrace, 1),
        '}' => (TokenType::CloseBrace, 1),
        '\'' => (TokenType::Quote, 1),
        '"' => {
            let mut string_end = start;
            loop {
                let string_length = string_end - start;
                if source[string_end] == '"' {
                    break (TokenType::String, string_length + 1)
                }
                if source.len() <= string_end {
                    break (TokenType::Error(ScanError::UnterminatedString),
                           string_length)
                }
                advance(source, &mut string_end, line);
            }
        }
        ':' => {
            let mut keyword_length = 1;
            while start + keyword_length < source.len()
                && is_symbol(source[start + keyword_length]) {
                    keyword_length += 1;
                }
            if keyword_length == 1 {
                (TokenType::Error(ScanError::EmptyKeyword), keyword_length)
            } else {
                (TokenType::Keyword, keyword_length)
            }
        }
        '-' => {
            let mut token_length = 1;
            while start + token_length < source.len()
                && is_number(source[start + token_length]) {
                    token_length += 1;
                }
            (TokenType::Number, token_length)
        }
        _ => {
            if start == source.len() - 1 {
                (TokenType::EOF, 1)
            } else if is_number(source[start]) {
                let mut token_length = 1;
                while start + token_length < source.len()
                    && is_number(source[start + token_length]) {
                        token_length += 1;
                    }
                (TokenType::Number, token_length)
            } else if is_symbol(source[start]) {
                let mut token_length = 1;
                while start + token_length < source.len()
                    && is_symbol(source[start + token_length]) {
                        token_length += 1;
                    }
                (TokenType::Symbol, token_length)
            } else {
                (TokenType::Error(ScanError::RanOff), 1)
            }
        },
    };
    Token {
        token_type: token_type,
        line: *line,
        start: start,
        length: length,
    }
}

pub fn scan(source: &Vec<char>, debug: bool) -> Vec<Token> {
    let mut offset = 0;
    let mut tokens = vec![];
    let mut line: usize = 1;
    loop {
        if offset >= source.len() {
            break tokens;
        }
        let token = scan_token(&source, offset, &mut line);
        if debug {
            println!("{:?} {} {} {} {}",
                     token.token_type,
                     token.line,
                     token.length,
                     token.start,
                     token.get_token(&source));
        }
        offset = token.start + token.length;
        tokens.insert(tokens.len(), token);
    }
}