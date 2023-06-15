#![allow(dead_code)]

use core::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub enum Tokens {
    /// Contains all valid Operators
    Op(Operator),
    /// Contains all valid keywords
    Kw(Keywords),
    ///  -> // comment I am
    Comment,
    /// Could be anything that identifies another thing
    /// let **id** = "identifier";
    Identifier,
    /// 'a';
    Char,
    /// "some string";
    String,
    /// any valid sequence of numbers that isn't a floating point number  
    Number,
    /// Any valid negative number '-11111'
    NegativeNumber,
    /// any valid sequence of numbers that is a floating point number
    FloatNumber,
    /// true
    BoolTrue,
    /// false
    BoolFalse,
    /// :
    Colon,
    /// ;
    SemiColon,
    /// ,
    Comma,
    /// !
    Bang,
    /// /
    Slash,
    /// .
    Dot,
    /// A tab
    Tab,
    /// (
    OpenBrace,
    /// )
    CloseBrace,
    /// {
    OpenCurlyBracket,
    /// }
    CloseCurlyBracket,
    /// [
    OpenBracket,
    /// ]
    CloseBracket,
    /// ^
    Pointer,
    /// Any token that is none of the above
    /// Contains a helper message for the user
    InvalidToken(TokenErrorMessages),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
    /// =
    Eq,
    /// ==
    EqEq,
    /// <
    Less,
    /// <=
    LessEq,
    /// '>'
    More,
    /// >=
    MoreEq,
    /// &
    And,
    /// &&
    AndAnd,
    /// |
    Or,
    /// ||
    OrOr,
    /// !=
    Nq,
    /// +
    Plus,
    /// -
    Min,
    /// *
    Times,
    /// +=
    PlusIs,
    /// *=
    TimesIs,
    /// -=
    MinusIs,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Keywords {
    /// let
    Let,
    /// if
    /// return
    Return,
    If,
    /// else
    Else,
    /// for
    For,
    /// while
    While,
    /// enum
    Enum,
    /// struct
    Struct,
    /// pub
    Pub,
    /// fn
    Fn,
    /// void
    Void,
    /// String
    String,
    /// char
    Char,
    /// I32
    I32,
    /// F32
    F32,
    /// U8
    U8,
    /// I8
    I8,
    // array
    Array,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenErrorMessages {
    /// This would be a string with no end -> [`"Hello world!`] <- missing [`'"'`] at the end    
    StringNoEnd,
    /// This would be a char with no direct '\'' after it: -> [`'c`] <- a char token should always have a closing '\''
    CharNoEnd,
    InvalidChar,
    /// This is for error messages with a token that doesn't exist.
    TokenInvalid(String),
}

impl Display for TokenErrorMessages {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", &self)
    }
}

impl ToString for Tokens {
    /// Converts the following types to their string value:
    ///
    /// Tokens::String => "string"
    /// Tokens::Number => "i32"
    /// Tokens::FloatNumber => "f32"
    /// Tokens::Char => "char"
    /// Tokens::Identifier => "identifier"
    fn to_string(&self) -> String {
        match self {
            Tokens::String => "string".into(),
            Tokens::Number => "i32".into(),
            Tokens::FloatNumber => "f32".into(),
            Tokens::Char => "char".into(),
            Tokens::Identifier => "identifier".into(),
            _ => "".into(),
        }
    }
}

impl std::error::Error for TokenErrorMessages {
    fn description(&self) -> &str {
        match &self {
            TokenErrorMessages::StringNoEnd => {
                "Found a string with no end, consider adding a \" to the end of the string"
            }
            TokenErrorMessages::CharNoEnd => {
                "Found a char with no end, consider adding a ' to the end of the char"
            }
            TokenErrorMessages::InvalidChar => "Found a invalid char",
            TokenErrorMessages::TokenInvalid(str) => str,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub line: usize,
    pub token_type: Tokens,
    pub value: String,
}

pub struct Tokenizer {
    chars: Vec<char>,
    current_position: usize,
    prev_char: Option<char>,
}

impl Iterator for Tokenizer {
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        let current_char = self.chars.get(self.current_position)?;
        self.prev_char = Some(*current_char);
        self.current_position += 1;
        Some(current_char.to_owned())
    }
}

impl Token {
    /// Creates a new [`Token`].
    ///
    /// It expects:
    ///
    /// The line on which the token was found,    
    ///
    /// The type of token being of Tokens::*
    ///
    /// The value of the token being either a direct token or for example a Tokens::String -> value is the string
    fn new(line: usize, token_type: Tokens, value: &str) -> Token {
        Token {
            line,
            token_type,
            value: value.to_string(),
        }
    }
}

impl Tokenizer {
    fn until_char(&self, char: char) -> Vec<char> {
        self.chars
            .iter()
            .take_while(|x| **x != char)
            .map(|x| x.to_owned())
            .collect()
    }
    fn until_char_mut(&mut self, char: char) -> Vec<char> {
        let mut up_until = Vec::new();
        for token in self.by_ref() {
            if token == char {
                break;
            }
            up_until.push(token);
        }
        up_until
    }
    /// walks back the iterator to pos - n
    fn advance_back(&mut self, n: usize) {
        self.current_position -= n;
        self.prev_char = self.chars.get(self.current_position).copied();
    }
}

pub trait Tokenize {
    /// returns a [`Tokens::String`] token
    /// Expects '"' to be the previous character
    fn token_str(&mut self, line: usize) -> Token;
    fn token_comment(&mut self, line: usize) -> Token;
    /// returns a [`Tokens::Char`] token
    /// Expects ''' to be the previous character
    fn token_char(&mut self, line: usize) -> Token;
    /// returns a [`Tokens::Op(Operator::Or || Operator::OrOr)`] token
    /// Expects ''' to be the previous character
    fn token_or(&mut self, line: usize) -> Token;
    /// returns a [`Tokens::Identifier`] || [`Tokens::Kw`]  token
    /// Expects the previous character to be of any letter type
    fn token_identifier(&mut self, line: usize) -> Token;
    /// returns a [`Tokens::Number`] || [`Tokens::FloatNumber`]  token
    /// Expects the previous character to be numeric
    fn token_num(&mut self, line: usize) -> Token;
    /// returns either a [`Tokens::Op(Operator::Eq)`] token or a [`Tokens::Op(Operator::EqEq)`] token
    /// Expects a '=' character to be the previous character
    fn token_eq(&mut self, line: usize) -> Token;
    /// returns either a [`Tokens::Op(Operator::Less)`] token or a [`Tokens::Op(Operator::LessEq)`] token
    /// Expects a '<' character to be the previous character
    fn token_less(&mut self, line: usize) -> Token;
    /// returns either a [`Tokens::Op(Operator::More)`] token or a [`Tokens::Op(Operator::MoreEq)`] token
    /// Expects a '<' character to be the previous character
    fn token_more(&mut self, line: usize) -> Token;
    /// returns either a [`Tokens::Bang`] token or a [`Tokens::Op(Operator::Nq)`] token
    /// Expects a '!' character to be the previous character
    fn token_bang(&mut self, line: usize) -> Token;
    /// returns either a [`Tokens::Op(Operator::And)`] token or a [`Tokens::Op(Operator::AndAnd)`] token
    /// Expects a '&' character to be the previous character
    fn token_and(&mut self, line: usize) -> Token;
    fn tokens_plus(&mut self, line: usize) -> Token;
    fn tokens_times(&mut self, line: usize) -> Token;
    fn tokens_minus(&mut self, line: usize) -> Token;
}

impl Tokenize for Tokenizer {
    fn token_str(&mut self, line: usize) -> Token {
        assert_eq!(self.prev_char, Some('"'));
        let tokens_until = self.until_char_mut('"');
        if tokens_until.is_empty() {
            return Token::new(
                line,
                Tokens::InvalidToken(TokenErrorMessages::StringNoEnd),
                "The string doesn't have a end",
            );
        };

        let str: String = tokens_until.into_iter().take_while(|x| *x != '"').collect();
        Token::new(line, Tokens::String, &str)
    }

    fn token_comment(&mut self, line: usize) -> Token {
        assert_eq!(self.prev_char, Some('/'));
        let Some(token_slash) = self.next() else {
           return Token::new(line, '/'.into(), "/");
        };
        match token_slash {
            '/' => {
                let tokens_until: String = self.until_char_mut('\n').iter().collect();
                return Token::new(line, Tokens::Comment, &tokens_until);
            }
            _ => {
                self.advance_back(1);
                return Token::new(line, '/'.into(), "/");
            }
        }
    }

    fn token_char(&mut self, line: usize) -> Token {
        assert_eq!(self.prev_char, Some('\''));
        let Some(tokens_until) = self.next() else {
            return Token::new(
                line,
                Tokens::InvalidToken(TokenErrorMessages::StringNoEnd),
                "The string doesn't have a end",
            );
        };

        match tokens_until {
            'a'..='z' | 'A'..='Z' | '\\' => {
                let char_close = self.next();

                match char_close.unwrap() {
                    'n' => {
                        let Some('\'') =  self.next() else {
                            return Token::new(line, Tokens::InvalidToken(TokenErrorMessages::CharNoEnd), &tokens_until.to_string(),);
                        };
                        return Token::new(line, Tokens::Char, "\n");
                    }
                    't' => {
                        let Some('\'') =  self.next() else {
                            return Token::new(line, Tokens::InvalidToken(TokenErrorMessages::CharNoEnd), &tokens_until.to_string(),);
                        };
                        return Token::new(line, Tokens::Char, "\t");
                    }
                    '\\' => {
                        let Some('\'') =  self.next() else {
                            return Token::new(line, Tokens::InvalidToken(TokenErrorMessages::CharNoEnd), &tokens_until.to_string(),);
                        };
                        return Token::new(line, Tokens::Char, "\\");
                    }
                    '0' => {
                        let Some('\'') =  self.next() else {
                            return Token::new(line, Tokens::InvalidToken(TokenErrorMessages::CharNoEnd), &tokens_until.to_string(),);
                        };
                        return Token::new(line, Tokens::Char, "\0");
                    }
                    _ => {}
                };

                if char_close == Some('\'') {
                    Token::new(line, Tokens::Char, &tokens_until.to_string())
                } else {
                    Token::new(
                        line,
                        Tokens::InvalidToken(TokenErrorMessages::CharNoEnd),
                        &tokens_until.to_string(),
                    )
                }
            }
            _ => Token::new(
                line,
                Tokens::InvalidToken(TokenErrorMessages::InvalidChar),
                &tokens_until.to_string(),
            ),
        }
    }

    fn token_identifier(&mut self, line: usize) -> Token {
        assert!(self.prev_char.is_some());
        assert!(self.prev_char.unwrap().is_alphabetic());

        let mut str = String::from(self.prev_char.unwrap());
        while let Some(char) = self.next() {
            match char {
                'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => str.push(char),
                _ => {
                    self.advance_back(1);
                    break;
                }
            }
        }
        Token::new(line, str.as_str().into(), &str)
    }

    fn token_num(&mut self, line: usize) -> Token {
        if let Some(prev) = self.prev_char {
            assert!(prev.is_numeric());
            #[allow(unused_variables)]
            let mut token_type = Tokens::Number;
            let mut nums = String::from(prev);
            while let Some(char) = self.next() {
                match char {
                    '0'..='9' => nums.push(char),
                    // if we find a dot anywhere inbetween the numbers we consider it to be a
                    // floating point number
                    #[allow(unused_assignments)]
                    '.' => {
                        token_type = Tokens::FloatNumber;
                        nums.push(char);
                    }
                    // We allow users to have underscores in their numbers for readability
                    '_' => continue,
                    _ => {
                        // We consider this to be the end and return the number token, we also make
                        // sure to advance back so that the char doesn't get consumed and can be
                        // used by the lexer
                        self.advance_back(1);
                        return Token::new(line, token_type, &nums);
                    }
                }
            }
            return Token::new(line, token_type, &nums);
        };

        Token::new(
            line,
            Tokens::InvalidToken(TokenErrorMessages::TokenInvalid(
                "Got a call to num but there was no previous number present.".to_string(),
            )),
            "no prev number",
        )
    }

    fn token_eq(&mut self, line: usize) -> Token {
        if let Some(prev) = self.prev_char {
            assert_eq!(prev, '=');
            match self.next() {
                Some(char) => {
                    match char {
                        '=' => return Token::new(line, "==".into(), "=="),
                        _ => {
                            // we advance the iterator back with 1 since the previous token is needed for the tokenizer
                            self.advance_back(1);
                            return Token::new(line, "=".into(), "=");
                        }
                    }
                }
                None => return Token::new(line, "=".into(), "="),
            }
        };

        Token::new(
            line,
            Tokens::InvalidToken(TokenErrorMessages::TokenInvalid(
                "Got a call to token_eq but there was no previous character present.".to_string(),
            )),
            "no prev token",
        )
    }

    fn token_less(&mut self, line: usize) -> Token {
        if let Some(prev) = self.prev_char {
            assert_eq!(prev, '<');
            match self.next() {
                Some(char) => {
                    match char {
                        '=' => return Token::new(line, "<=".into(), "<="),
                        _ => {
                            // we advance the iterator back with 1 since the previous token is needed for the tokenizer
                            self.advance_back(1);
                            return Token::new(line, "<".into(), "<");
                        }
                    }
                }
                None => return Token::new(line, "<".into(), "<"),
            }
        };

        Token::new(
            line,
            Tokens::InvalidToken(TokenErrorMessages::TokenInvalid(
                "Got a call to token_less but there was no previous character present.".to_string(),
            )),
            "no prev token",
        )
    }

    fn token_more(&mut self, line: usize) -> Token {
        if let Some(prev) = self.prev_char {
            assert_eq!(prev, '>');
            match self.next() {
                Some(char) => {
                    match char {
                        '=' => return Token::new(line, ">=".into(), ">="),
                        _ => {
                            // we advance the iterator back with 1 since the previous token is needed for the tokenizer
                            self.advance_back(1);
                            return Token::new(line, ">".into(), ">");
                        }
                    }
                }
                None => return Token::new(line, ">".into(), ">"),
            }
        };

        Token::new(
            line,
            Tokens::InvalidToken(TokenErrorMessages::TokenInvalid(
                "Got a call to token_more but there was no previous character present.".to_string(),
            )),
            "no prev token",
        )
    }

    fn token_bang(&mut self, line: usize) -> Token {
        if let Some(prev) = self.prev_char {
            assert_eq!(prev, '!');
            match self.next() {
                Some(char) => {
                    match char {
                        '=' => return Token::new(line, "!=".into(), "!="),
                        _ => {
                            // we advance the iterator back with 1 since the previous token is needed for the tokenizer
                            self.advance_back(1);
                            return Token::new(line, "!".into(), "!");
                        }
                    }
                }
                None => return Token::new(line, "!".into(), "!"),
            }
        };

        Token::new(
            line,
            Tokens::InvalidToken(TokenErrorMessages::TokenInvalid(
                "Got a call to token_bang but there was no previous character present.".to_string(),
            )),
            "no prev token",
        )
    }

    fn token_and(&mut self, line: usize) -> Token {
        if let Some(prev) = self.prev_char {
            assert_eq!(prev, '&');
            match self.next() {
                Some(char) => {
                    match char {
                        '&' => return Token::new(line, "&&".into(), "&&"),
                        _ => {
                            // we advance the iterator back with 1 since the previous token is needed for the tokenizer
                            self.advance_back(1);
                            return Token::new(line, "&".into(), "&");
                        }
                    }
                }
                None => return Token::new(line, "&".into(), "&"),
            }
        };

        Token::new(
            line,
            Tokens::InvalidToken(TokenErrorMessages::TokenInvalid(
                "Got a call to token_and but there was no previous character present.".to_string(),
            )),
            "no prev token",
        )
    }

    fn token_or(&mut self, line: usize) -> Token {
        let Some(next) = self.next() else {
            self.advance_back(1);
            return Token::new(line, "|".into(), "|");
        };
        match next {
            '|' => Token::new(line, "||".into(), "||"),
            _ => {
                self.advance_back(1);
                Token::new(line, "|".into(), "|")
            }
        }
    }

    fn tokens_plus(&mut self, line: usize) -> Token {
        if let Some(prev) = self.prev_char {
            assert_eq!(prev, '+');
            match self.next() {
                Some(char) => match char {
                    '=' => {
                        return Token::new(line, "+=".into(), "+=");
                    }
                    _ => {
                        self.advance_back(1);
                        return Token::new(line, '+'.into(), "+");
                    }
                },
                None => {
                    return Token::new(line, '+'.into(), "+");
                }
            }
        }
        Token::new(
            line,
            Tokens::InvalidToken(TokenErrorMessages::TokenInvalid(
                "Got a call to token_plus but there was no previous character present.".to_string(),
            )),
            "no prev token",
        )
    }

    fn tokens_times(&mut self, line: usize) -> Token {
        if let Some(prev) = self.prev_char {
            assert_eq!(prev, '*');
            match self.next() {
                Some(char) => match char {
                    '=' => {
                        return Token::new(line, "*=".into(), "*=");
                    }
                    _ => {
                        self.advance_back(1);
                        return Token::new(line, '*'.into(), "*");
                    }
                },
                None => {
                    return Token::new(line, '*'.into(), "*");
                }
            }
        }
        Token::new(
            line,
            Tokens::InvalidToken(TokenErrorMessages::TokenInvalid(
                "Got a call to token_times but there was no previous character present."
                    .to_string(),
            )),
            "no prev token",
        )
    }

    fn tokens_minus(&mut self, line: usize) -> Token {
        if let Some(prev) = self.prev_char {
            assert_eq!(prev, '-');
            match self.next() {
                Some(char) => match char {
                    '=' => {
                        return Token::new(line, "-=".into(), "-=");
                    }
                    '0'..='9'  => {
                        let mut str_nums = String::from(char);
                        while let Some(char) = self.next() {
                            match char {
                                '0'..='9' => str_nums.push(char),
                                _ => {
                                    self.advance_back(1);
                                    return Token::new(line, Tokens::NegativeNumber, &str_nums);
                                }
                            }
                        }
                        return Token::new(line, Tokens::NegativeNumber, &str_nums);
                    }
                    _ => {
                        self.advance_back(1);
                        return Token::new(line, '-'.into(), "-");
                    }
                },
                None => {
                    return Token::new(line, '-'.into(), "-");
                }
            }
        }
        Token::new(
            line,
            Tokens::InvalidToken(TokenErrorMessages::TokenInvalid(
                "Got a call to token_minus but there was no previous character present."
                    .to_string(),
            )),
            "no prev token",
        )
    }
}

pub trait Lexer {
    fn new(chars: &str) -> Self;

    fn lex(tokenizer: &mut Tokenizer) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut line: usize = 0;
        while let Some(char) = tokenizer.next() {
            match char {
                '\n' => line += 1,
                ' ' => continue,
                '"' => tokens.push(tokenizer.token_str(line)),
                '=' => tokens.push(tokenizer.token_eq(line)),
                '!' => tokens.push(tokenizer.token_bang(line)),
                '<' => tokens.push(tokenizer.token_less(line)),
                '>' => tokens.push(tokenizer.token_more(line)),
                '\'' => tokens.push(tokenizer.token_char(line)),
                '|' => tokens.push(tokenizer.token_or(line)),
                '&' => tokens.push(tokenizer.token_and(line)),
                '0'..='9' => tokens.push(tokenizer.token_num(line)),
                'a'..='z' | 'A'..='Z' => tokens.push(tokenizer.token_identifier(line)),
                '/' => tokens.push(tokenizer.token_comment(line)),
                '+' => tokens.push(tokenizer.tokens_plus(line)),
                '*' => tokens.push(tokenizer.tokens_times(line)),
                '-' => tokens.push(tokenizer.tokens_minus(line)),
                token => {
                    let token = Token::new(line, token.into(), &token.to_string());
                    if let Tokens::InvalidToken(_) = token.token_type {
                        continue;
                    }
                    tokens.push(token)
                }
            };
        }
        tokens
    }
}

impl Lexer for Tokenizer {
    fn new(char: &str) -> Self {
        Tokenizer {
            chars: char.chars().collect(),
            current_position: 0,
            prev_char: None,
        }
    }
}
