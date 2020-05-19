#[derive(Debug)]
pub enum Token {
    Lparen,
    Rparen,
    Number(usize),
    Symbol([u8; 2]),
    InvalidToken(usize),
}

pub struct Tokenizer<'a> {
    source: &'a [u8],
    position: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(source: &'a str) -> Tokenizer<'a> {
        Tokenizer {
            source: source.as_bytes(),
            position: 0,
        }
    }
    fn consume_number(&mut self) -> usize {
        let mut value = 0_usize;
        while let Some(&chr) = self.source.get(self.position) {
            if chr >= b'0' && chr <= b'9' {
                self.position += 1;
                value = value * 10 + ((chr - b'0') as usize);
            } else {
                break;
            }
        }
        value
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        use Token::*;
        let src = self.source;
        let mut pos = self.position;
        while let Some(b' ') = src.get(pos) {
            pos += 1;
        }
        src.get(pos).map(|&head| {
            match head {
                b'0'..=b'9' => {
                    self.position = pos;
                    Number(self.consume_number())
                }
                b'(' => {
                    self.position = pos + 1;
                    Lparen
                }
                b')' => {
                    self.position = pos + 1;
                    Rparen
                }
                b'A'..=b'Z' => {
                    pos += 1;
                    let mut sym = [head, 0];
                    if let Some(&next) = src.get(pos) {
                        if next >= b'a' && next <= b'z' {
                            sym[1] = next;
                            pos += 1;
                        }
                    }
                    self.position = pos;
                    Symbol(sym)
                }
                b'a'..=b'z' => {
                    self.position = pos + 1;
                    Symbol([head.to_ascii_uppercase(), 0])
                }
                _ => InvalidToken(pos)
            }
        })
    }
}
