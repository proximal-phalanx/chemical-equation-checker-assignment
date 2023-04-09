use crate::token::Token;

pub struct TokenStream {
    source: Vec<u8>,
    index: usize,
}

impl TokenStream {
    fn next_char(&mut self) -> u8 {
        let c = self.source[self.index];
        self.index += 1;
        c
    }
    fn peek_char(&self) -> u8 {
        self.source[self.index]
    }
    fn eof_char(&self) -> bool {
        self.index >= self.source.len()
    }
}

impl TokenStream {
    pub fn new(src: String) -> TokenStream {
        TokenStream { source: src.as_bytes().to_vec(), index: 0}    
    }
    pub fn next(&mut self) -> Option<Token> {
        if self.eof_char() {
            return None;
        }
        let cur = self.next_char();
        match cur {
            b' ' | b'\n' => {
                return self.next()
            }
            b'0'..=b'9' => {
                let mut num = (cur - b'0') as i128;
                while !self.eof_char() && self.peek_char().is_ascii_digit() {
                    num = num * 10 + (self.next_char() - b'0') as i128;
                }
                Some(Token::Number(num))
            }
            b'+' => Some(Token::Plus),
            b'=' => Some(Token::Equal),
            b'(' => Some(Token::LPara),
            b')' => Some(Token::RPara),
            b'A'..=b'Z' => {
                let mut element = [cur, 0];
                if !self.eof_char() && self.peek_char().is_ascii_lowercase() {
                    element[1] = self.next_char();
                }
                Some(Token::Element(element))
            }
            _ => panic!("unexpected character: {}", cur as char),
        }
    }
}