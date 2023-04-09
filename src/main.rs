use std::collections::HashMap;

#[cfg(test)]
mod test;
mod token_stream;
mod counter;
mod token;
mod term_stream;

fn main() {
    let mut num_string = String::new();
    std::io::stdin().read_line(&mut num_string).unwrap();
    let num = num_string.trim().parse::<usize>().unwrap();
    for _ in 0..num {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let ts = token_stream::TokenStream::new(buffer);
        let mut term_str = term_stream::TermStream::new(ts);
        let mut cnt: HashMap<[u8; 2], i128> = HashMap::new();
        counter::count(&mut term_str, &mut cnt);
        if counter::all_zero(&cnt) {
            println!("Y");
        } else {
            println!("N");
        }
    }
}
