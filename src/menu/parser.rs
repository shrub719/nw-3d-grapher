pub enum ParserError {

}

pub enum Token {

}

pub fn lex(input: &mut Chars) -> Result<Vec<Token>, ParserError> {
    let mut tokens = Vec::new();
    let mut pos: usize = 0;
    let mut running_id = String::new();

    while let Some(ch) = input.next() {
        pos += 1
    }
}
