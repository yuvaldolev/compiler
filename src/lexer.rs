pub struct Lexer {
    chars: Chars,

    index: usize,
    line: usize,
    column: usize,

    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(source: String) -> Lexer {
        let result = Lexer {
            chars: source.chars().peekable(),
            index: 0,
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

        if let Some(&at) = self.chars.peek() {
            let start_index = self.index;
            self.advance();

            // NOTE(ydolev): There's still some characters to be lexed!
            if Self::starts_identifier(at) {
                // NOTE(ydolev): This is an identifier or a keyword.
                // NOTE(ydolev): Read the whole string.
                while let Some(&at) = self.chars.peek() {
                    if !self.continues_identifier(at) {
                        break;
                    }
                    self.advance();
                }

                // TODO(ydolev): Speed this up - use some sort of a
                // hash map to map between the text and its type.
                // This is a rather bad way of matching keywords,
                // but its fast (in development time) and it works for now.
                let source = &self.source[start_index..self.index];
                match source {
                    // TODO(ydolev): Add all keywords here.
                    "proc" => result.kind = TokenKind::KeywordProc,
                    "if" => result.kind = TokenKind::KeywordIf,
                    _ => result.kind = TokenKind::Identifier,
                }
            }
        } else {
            // NOTE(ydolev): Reached EOF.
            result.kind = TokenKind::EOF;
        }

        result
    }

    fn eat_whitespace(&mut self) {
        while let Some(&c) = self.chars.peek() {
            self.advance();
        }
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.chars.next()?;
        self.index += 1;
        self.column += 1;
        Some(c)
    }

    fn starts_identifier(c: char) -> bool {
        ('_' == c) || c.is_alphabetic()
    }

    fn continues_identifier(c: char) -> bool {
        Self::starts_identifier(c) || c.is_numeric()
    }
}
