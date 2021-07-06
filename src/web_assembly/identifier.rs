/// Indices can be given in both numeric and symbolic form.
/// Symbolic identifiers that stand in lieu of indices start with ‘$’,
/// followed by any sequence of printable ASCII characters that does not contain a space,
/// quotation mark, comma, semicolon, or bracket.
/// ‘𝟶’  |  …  |  ‘𝟿’
/// ‘𝙰’  |  …  |  ‘𝚉’
/// ‘𝚊’  |  …  |  ‘𝚣’
/// ‘!’  |  ‘#’  |  ‘$’  |  ‘%’  |  ‘&’  |  ‘′’  |  ‘∗’  |  ‘+’  |  ‘−’  |  ‘.’  |  ‘/’
/// ‘:’  |  ‘<’  |  ‘=’  |  ‘>’  |  ‘?’  |  ‘@’  |  ‘∖’  |  ‘^’  |  ‘_’  |  ‘`’  |  ‘|’  |  ‘~’
#[derive(Clone)]
pub struct Identifier {
    id: Option<SymbolicIdentifier>,
    index: usize,
}

#[derive(Clone)]
pub struct SymbolicIdentifier {
    bytes: Vec<u8>,
}
