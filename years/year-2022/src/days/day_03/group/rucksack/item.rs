use nutype::nutype;

#[nutype(derive(Debug, Clone, Copy, PartialEq, Eq, Hash))]
pub struct Item(char);
