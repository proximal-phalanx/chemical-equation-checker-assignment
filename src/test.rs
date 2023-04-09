use std::collections::HashMap;

use crate::token::Token;

#[cfg(test)]
const REACTIONS: [&str; 10] = [
    "2H2O=2H2+O2",
    "2Cu+4HCl=2H(CuCl2)+H2",
    "AgNO3+2NH3=(Ag(NH3)2)NO3",
    "13P4+10P2I4+128H2O=40PH4I+32H3PO4",
    "4Au+8NaCN+2H2O+O2=4Na(Au(CN)2)+4NaOH",
    "2K2Cr2O7+8H2SO4+3CH3CH2OH=3CH3COOH+2Cr2(SO4)3+11H2O+2K2SO4",
    "CH2OH(CHOH)4CHO+2Ag(NH3)2OH=CH2OH(CHOH)4COONH4+2Ag+3NH3+H2O",
    "K3(Fe(CN)6)+6HCl=3KCl+FeCl3+6HCN",
    "K4(Fe(CN)6)+3H2SO4=2K2SO4+FeSO4+6HCN",
    "((NH4)3PO4)(MoO3)12+27NaOH=3NH3+Na3PO4+12Na2MoO4+15H2O"
];

#[test]
fn tokenization_test() {
    for reaction in REACTIONS.iter() {
        println!("[Case Input]");
        println!("{}", reaction);
        let mut ts = crate::token_stream::TokenStream::new(reaction.to_string());
        println!("[Case Output]");
        while let Some(tk) = ts.next() {
            match tk {
                Token::Number(n) => println!("{}", n),
                Token::Plus => println!("+"),
                Token::Equal => println!("="),
                Token::Element(e) => println!("{}", String::from_utf8(e.to_vec()).unwrap()),
                Token::LPara => println!("("),
                Token::RPara => println!(")"),
            }
        }
        println!("[Case End]");
    }
}

#[test]
fn term_parsing_test() {
    for reaction in REACTIONS.iter() {
        println!("[Case Input]");
        println!("{}", reaction);
        let token_str = crate::token_stream::TokenStream::new(reaction.to_string());
        println!("[Case Output]");
        let mut term_str = crate::term_stream::TermStream::new(token_str);
        while !term_str.eof() {
            println!("[Term Start]");
            let sub = term_str.next();
            for tk in sub {
                match tk {
                    Token::Element(e) => println!("{}", String::from_utf8(e.to_vec()).unwrap()),
                    Token::LPara => println!("("),
                    Token::RPara => println!(")"),
                    Token::Number(n) => println!("{}", n),
                    _ => panic!("Invalid token"),
                }
            }
            println!("[Term End]");
        }
        println!("[Case End]");
    }
}

#[test]
fn counting_test() {
    for reaction in REACTIONS.iter() {
        println!("[Case Input]");
        println!("{}", reaction);
        let token_str = crate::token_stream::TokenStream::new(reaction.to_string());
        println!("[Case Output]");
        let mut term_str = crate::term_stream::TermStream::new(token_str);
        let mut cnt: HashMap<[u8; 2], i128> = HashMap::new();
        crate::counter::count(&mut term_str, &mut cnt);
        for (k, v) in cnt.iter() {
            println!("{}: {}", String::from_utf8(k.to_vec()).unwrap(), v);
            assert_eq!(*v, 0);
        }
        println!("[Case End]");
    }
}

#[cfg(test)]
const UNEQUAL_REACTIONS: [&str; 5] = [
    "2((((((H))))))=2((O)2)",
    "C(CO(OH)COOP)KK2Li=(C)2(As)2(P)1(Li)(KO100)3",
    "23((NH4)3PO4)(MoO3)12+27NaOH=3NH3+Na3PO4+12Na2MoO4+15H2O",
    "(COOH)2(OOOO)2(O1000O2000O100(K(KO(K(KO)OO113)2)))=C",
    "(OOP)2(AsLi20)3(K(O2))8=O",
];

#[test]
fn counting_unequal_test() {
    for reaction in UNEQUAL_REACTIONS.iter() {
        println!("[Case Input]");
        println!("{}", reaction);
        let token_str = crate::token_stream::TokenStream::new(reaction.to_string());
        println!("[Case Output]");
        let mut term_str = crate::term_stream::TermStream::new(token_str);
        let mut cnt: HashMap<[u8; 2], i128> = HashMap::new();
        crate::counter::count(&mut term_str, &mut cnt);
        let mut eq = true;
        for (k, v) in cnt.iter() {
            println!("{}: {}", String::from_utf8(k.to_vec()).unwrap(), v);
            if *v != 0 {
                eq = false;
            }
        }
        assert_eq!(eq, false);
        println!("[Case End]");
    }
}