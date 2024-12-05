use std::cmp::PartialEq;
use std::hash::Hash;

#[derive(Debug)]
enum ParserError {
    InvalidInput,
}

#[derive(Debug, PartialEq, Eq)]
enum ParserState {
    None,
    MFound,
    UFound,
    LFound,
    OpenBrace,
    CloseBrace,
    NumFound,
    CommaFound
}

impl ParserState {
    fn next_possible_state(&self, chr :char) -> ParserState {
        if chr == 'm' && *self == ParserState::None {
            return ParserState::MFound
        }
        if chr == 'u' && *self == ParserState::MFound {
            return ParserState::UFound
        }
        if chr == 'l' && *self == ParserState::UFound {
            return ParserState::LFound
        }
        if chr == '(' && *self == ParserState::LFound {
            return ParserState::OpenBrace
        }
        if chr == ')' && *self == ParserState::NumFound {
            return ParserState::CloseBrace
        }
        if chr == ',' && *self == ParserState::NumFound {
            return ParserState::CommaFound
        }
        if chr.is_digit(10) &&
            *self == ParserState::NumFound || *self == ParserState::CommaFound || *self == ParserState::OpenBrace {
            return ParserState::NumFound
        }
        ParserState::None
    }
}

struct Multiplier {
    state: ParserState,
    left_num: Option<String>,
    right_num: Option<String>,
    left_complete: bool
}

impl Multiplier {
    fn new() -> Multiplier {
        Multiplier {
            state: ParserState::None,
            left_num: None,
            right_num: None,
            left_complete: false
        }
    }

    fn read(&mut self, chr: char) -> Result<(), ParserError> {
        let next_state = self.state.next_possible_state(chr);
        if next_state == ParserState::None {
            Err(ParserError::InvalidInput)
        } else {
            self.state = next_state;
            if self.state == ParserState::NumFound {
                if !self.left_complete {
                    if self.left_num.is_none() {
                        self.left_num = Some(String::from(chr));
                    } else {
                        self.left_num.as_mut().unwrap().push(chr);
                    }
                } else {
                    if self.right_num.is_none() {
                        self.right_num = Some(String::from(chr));
                    } else {
                        self.right_num.as_mut().unwrap().push(chr);
                    }
                }
            }
            if self.state == ParserState::CommaFound {
                self.left_complete = true;
            }
            Ok(())
        }
    }

    fn get_result(&self) -> u128 {
        if self.state != ParserState::CloseBrace {
            return 0;
        }
        let default_val = String::from("0");
        let left_num = self.left_num.as_ref().unwrap_or(&default_val).parse::<u128>().unwrap_or_else(|_| 0);
        let right_num = self.right_num.as_ref().unwrap_or(&default_val).parse::<u128>().unwrap_or_else(|_| 0);

        left_num * right_num
    }
}

fn main() {
    let input = include_str!("../puzzle_input.txt");

    let mut result: u128 = 0;
    let mut multiplier = Multiplier::new();

    for chr in input.chars() {
        match multiplier.read(chr) {
            Ok(_) => {
                if multiplier.state == ParserState::CloseBrace {
                    result += multiplier.get_result();
                    multiplier = Multiplier::new();
                }
            },
            Err(e) => {
                result += multiplier.get_result();
                multiplier = Multiplier::new()
            }
        }
    }

    println!("Result: {}", result);
}
