use std::collections::HashMap;

use crate::{term_stream::TermStream, token::Token};

pub fn count(ts: &mut TermStream, cnt: &mut HashMap<[u8; 2], i128>) {
    while !ts.eof() {
        let term = ts.next();
        let mut stk_token: Vec<Token> = vec![];
        let mut stk_coef: Vec<i128> = vec![];
        let mut coef = 1;
        for tk in term {
            stk_token.push(tk);
        }
        while let Some(tk) = stk_token.pop() {
            match tk {
                Token::Number(n) => {
                    coef *= n;
                    stk_coef.push(n);
                }
                Token::LPara => {
                    let top = stk_coef.pop().unwrap();
                    coef /= top;
                }
                Token::Element(elem) => {
                    let it = cnt.entry(elem).or_insert(0);
                    *it += coef;
                }
                Token::RPara => {
                    continue;
                }
                Token::Equal | Token::Plus => {
                    panic!("Invalid token.");
                }
            }
        }
    }
}