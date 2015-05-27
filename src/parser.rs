pub struct Parser<'a> {
    str: &'a str,
    pos: usize
}

impl<'a> Parser<'a> {
    pub fn new(str: &'a str) -> Parser<'a> {
        Parser {
            str: str,
            pos: 0
        }
    }
}
