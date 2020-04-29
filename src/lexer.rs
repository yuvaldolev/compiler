pub struct Lexer {
    source: &String,
    chars: Chars,

    index: usize,
    line: usize,
    column: usize,

    tokens: Vec<Token>,
}

pub struct Token {}

impl Token {}

pub enum TokenKind {}

enum Radix {
    Bin = 2,
    Oct = 8,
    Dec = 10,
    Hex = 16,
}

const EOF_CHAR: char = '\0';

impl Lexer {
    pub fn new(source: &String) -> Lexer {
        let result = Lexer {
            source,
            chars: source.chars(),
            line: 1,
            column: 1,
            tokens: Vec::new(),
        };
    }

    pub fn lex(&mut self) {
        loop {
            let token = self.lex_token();
            if (token.kind == TokenKind::Comment) {
                // NOTE(ydolev): Skip comments.
                continue;
            }
            if (token.kind == TokenKind::EOF) {
                // NOTE(ydolev): Reached end of file. break.
                break;
            }

            self.tokens.push(token);
        }
    }

    fn lex_token(&mut self) -> Token {
        self.eat_whitespace();

        let result = Token::new();
        result.line = self.line;
        result.column = self.column;

        if self.is_eof() {
            // NOTE(ydolev): Reached EOF.
            result.kind = TokenKind::EOF;
            return result;
        }

        // NOTE(ydolev): There's still some characters to be lexed!
        let at = self.peek_first();
        let start_index = self.index();
        self.advance();

        if Self::is_identifier_start(at) {
            // NOTE(ydolev): This is an identifier or a keyword.
            // NOTE(ydolev): Read the whole string.
            while Self::is_identifier_continue(self.peek_first()) {
                self.advance();
            }
            let source = &self.source[start_index..self.index()];

            // TODO(ydolev): Speed this up - use some sort of a
            // hash map to map between the text and its type.
            // This is a rather bad way of matching keywords,
            // but its fast (in development time) and it works for now.
            match source {
                // TODO(ydolev): Add all keywords here.
                "proc" => result.kind = TokenKind::KeywordProc,
                "if" => result.kind = TokenKind::KeywordIf,
                "i32" => result.kind = TokenKind::KeywordI32,
                _ => result.kind = TokenKind::Identifier,
            }
        } else if (at.is_numeric()) {
            // NOTE(ydolev): This is a number.
            let radix = Redix::Dec;
            if ('0' == at) {
                at = self.peek_first();

                // NOTE(ydolev): Find the number's base.
                if ('b' == at) || ('B' == at) {
                    radix = Redix::Bin;
                    self.advance();
                } else if ('o' == at) || ('O' == at) {
                    radix = Redix::Oct;
                    self.advance();
                } else if ('x' == at) || ('X' == at) {
                    radix = Redix::Hex;
                    self.advance();
                }
            }

            // NOTE(ydolev): Read all digits.
            self.eat_digits(radix as u32);

            // NOTE(ydolev): If the next character is a dot, validate that
            // the character after it is a digit, if not, than this is not
            // a floating point number.
            if ('.' == self.peek_first())
                && (Redix::Dec == radix)
                && self.peek_second().is_digit(Redix::Dec as u32)
            {
                self.advance();

                // NOTE(ydolev): Read all floating point digits.
                self.eat_digits(Redix::Dec as u32);

                // NOTE(ydolev): Read all scientific notation chars.
                if (('e' == self.peek_first()) || ('E' == self.peek_first())) {
                    self.advance();

                    // NOTE(ydolev): Read exponent sign.
                    if ('+' == self.peek_first()) || ('-' == self.peek_first()) {
                        self.advance();
                    }

                    // NOTE(ydolev): Read all exponent digits.
                    self.eat_digits(Redix::Dec as u32);
                }

                token.kind = TokenKind::Float;
                let source = &self.source[start_index..self.index()];
                token.float = source.parse().unwrap_or(|err| {
                    // TODO(ydolev): Report error here.
                    0
                });
            } else {
                token.kind = TokenKind::Integer;
                let source = &self.source[start_index..self.index()];
                token.int = source.parse().unwrap_or_else(|err| {
                    // TODO(ydolev): Report error here.
                    0
                });
            }
        } else {
            match at {
                ';' => result.kind = TokenKind::Semi,
                ',' => result.kind = TokenKind::Comma,
                '.' => result.kind = TokenKind::Dot,
                '(' => result.kind = TokenKind::OpenParen,
                ')' => result.kind = TokenKind::CloseParen,
                '{' => result.kind = TokenKind::OpenBrace,
                '}' => result.kind = TokenKind::CloseBrace,
                '[' => result.kind = TokenKind::OpenBracket,
                ']' => result.kind = TokenKind::CloseBracket,

                '"' => {
                    // TODO(ydolev): Lex the string.
                }

                '+' => {
                    if '=' == self.peek_first() {
                        self.advance();
                        result.kind = TokenKind::PlusEq;
                    } else {
                        result.kind = TokenKind::Plus;
                    }
                }

                '-' => {
                    if '>' == self.peek_first() {
                        self.advance();
                        result.kind = TokenKind::Arrow;
                    } else if ('=' == self.peek_first()) {
                        self.advance();
                        result.kind = TokenKind::MinusEq;
                    } else {
                        result.kind = TokenKind::Minus;
                    }
                }

                '*' => {
                    if '=' == self.peek_first() {
                        self.advance();
                        result.kind = TokenKind::StarEq;
                    } else {
                        result.kind = TokenKind::Star;
                    }
                }

                '/' => {
                    // TODO(ydolev): Lex comments.
                    if '=' == self.peek_first() {
                        self.advance();
                        result.kind = TokenKind::SlashEq;
                    } else {
                        result.kind = TokenKind::Slash;
                    }
                }

                ':' => {
                    if ':' == self.peek_first() {
                        self.advance();
                        result.kind = TokenKind::ColonColon;
                    } else {
                        result.kind = TokenKind::Colon;
                    }
                }
            }
        }

        result
    }

    fn peek_first(&self) -> char {
        self.peek_nth(0)
    }

    fn peek_second(&self) -> char {
        self.peek_nth(1)
    }

    fn peek_nth(&self, n: usize) -> char {
        self.chars.nth(n).unwrap_or(EOF_CHAR)
    }

    fn advance(&mut self) {
        self.chars.next();
        self.column += 1;
    }

    fn index(&self) -> usize {
        self.source.len() - self.chars.as_str().len()
    }

    fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    fn eat_digits(&mut self) {
        while self.peek_first().is_digit(radix) {
            self.advance();
        }
    }

    fn eat_whitespace(&mut self) {
        while self.peek_first().is_whitespace() {
            self.advance();
        }
    }

    fn is_identifier_start(c: char) -> bool {
        ('_' == c) || c.is_alphabetic()
    }

    fn is_identifier_continue(c: char) -> bool {
        Self::starts_identifier(c) || c.is_numeric()
    }
}
