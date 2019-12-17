pub trait New {
    fn new(input: &[u8]) -> Self;
}

pub trait Hash {
    fn hash(_: &[u8]) -> String;
}
