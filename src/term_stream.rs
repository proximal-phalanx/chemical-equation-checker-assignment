use crate::{token::Token, token_stream::TokenStream};

pub struct TermStream {
    source: Vec<Token>,
    index: usize,
    neg: bool,
}

impl TermStream {
    pub fn new(mut ts: TokenStream) -> TermStream {
        let mut src: Vec<Token> = Vec::new();
        while let Some(tk) = ts.next() {
            src.push(tk);
        }
        TermStream { source: src, index: 0, neg: false }
    }
}

impl TermStream {
    pub fn eof(&mut self) -> bool {
        self.index >= self.source.len()
    }
    fn next_token(&mut self) -> Token {
        let tk = self.source[self.index].clone();
        self.index += 1;
        tk
    }
    fn peek_token(&self) -> Token {
        self.source[self.index].clone()
    }
    fn cur_token(&self) -> Token {
        if self.index == 0 {
            Token::Plus
        }
        else{
            self.source[self.index - 1].clone()
        }
    }
}

impl TermStream {
    pub fn next(&mut self) -> Vec<Token> {
        if self.eof() {
            panic!("EOF");
        }
        let tk_first = self.peek_token();
        let mut coef: i128 = if self.neg { -1 } else { 1 };
        match tk_first {
            Token::Number(c) => {
                coef *= c;
                self.next_token();
            }
            _ => {}
        }
        let prefix = vec![Token::LPara];
        let suffix = vec![Token::RPara, Token::Number(coef)];
        let mut mid: Vec<Token> = Vec::new();
        while !self.eof() {
            let tk = self.next_token();
            match tk {
                Token::LPara | Token::Number(_) => {
                    mid.push(tk);
                }
                Token::Plus => {
                    break;
                }
                Token::Equal => {
                    self.neg = true;
                    break;
                }
                Token::RPara => {
                    let next_token = if self.eof() {
                        Token::Plus
                    } else {
                        self.peek_token()
                    };
                    match next_token {
                        Token::Number(_) => {
                            mid.push(tk);
                        }
                        _ => {
                            mid.push(tk);
                            mid.push(Token::Number(1));
                        }
                    }
                }
                Token::Element(_) => {
                    let next_token = if self.eof() {
                        Token::Plus
                    } else {
                        self.peek_token()
                    };
                    match next_token {
                        Token::Number(_) => {
                            mid.push(Token::LPara);
                            mid.push(tk);
                            mid.push(Token::RPara);
                        }
                        _ => {
                            mid.push(Token::LPara);
                            mid.push(tk);
                            mid.push(Token::RPara);
                            mid.push(Token::Number(1));
                        }
                    }
                }
            }
        }
        let mut res = Vec::new();
        res.extend(prefix);
        res.extend(mid);
        res.extend(suffix);
        res
    }
}