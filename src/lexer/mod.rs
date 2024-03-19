
struct Lexer {
    src:String,
    /// This is the raw source code
}

impl Lexer {
    /// Converts self.src into an itterable of strings.
    fn tokenize_strings(&mut self) -> Vec<String> {
        for c in self.src.chars() {
            match c {
                '0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9' => {
                    //num
                }
                '"'|'\'' => {
                    //string
                }
                _ => {
                    //label
                }
            };
        }
    }
}