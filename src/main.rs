#[cfg(test)]
mod test;
mod token_stream;
mod count;
mod token;
mod term_stream;

fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let mut ts = token_stream::TokenStream::new(buffer);
}
