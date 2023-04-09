#[derive(Debug, PartialEq, Eq, Hash, Clone)]
/// for element with only one character, like H,
/// the array stores ['H', '\0']
pub enum Token {
    Number(i128),
    Plus,
    Equal,
    Element([u8; 2]),
    LPara,
    RPara,
}