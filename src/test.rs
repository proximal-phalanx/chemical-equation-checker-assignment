use crate::token::Token;

#[test]
fn tokenization_test() {
    let reactions = [
        "2H2O = 2H2 + O2",
        "2Cu+4HCl=2H(CuCl2)+H2",
        "AgNO3+2NH3=(Ag(NH3)2)NO3",
        "13P4+10P2I4+128H2O = 40PH4I + 32H3PO4\n",
        "2K2Cr2O7+ 36Fe3O4+ 62H2SO4=2K2SO4+ 18Fe2(SO4)3+ 2Cr2(SO4)3+ 31H2O",
        "4Au+8NaCN+2H20+02=4Na (Au(CN)2)+4NaOH"
    ];
    for reaction in reactions.iter() {
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
    let reactions = [
        "2H2O = 2H2 + O2",
        "2Cu+4HCl=2H(CuCl2)+H2",
        "AgNO3+2NH3=(Ag(NH3)2)NO3",
        "13P4+10P2I4+128H2O = 40PH4I + 32H3PO4\n",
        "2K2Cr2O7+ 36Fe3O4+ 62H2SO4=2K2SO4+ 18Fe2(SO4)3+ 2Cr2(SO4)3+ 31H2O",
        "4Au+8NaCN+2H20+02=4Na (Au(CN)2)+4NaOH"
    ];
    for reaction in reactions.iter() {
        println!("[Case Input]");
        println!("{}", reaction);
        let mut token_str = crate::token_stream::TokenStream::new(reaction.to_string());
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