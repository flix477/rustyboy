use std::io::{self, Write};
use console::{Term, Key};

enum Action {
    Back,
    Forward,
    Char(char),
    Submit,
    Backspace
}

pub struct Shell {
    history: Log
}

impl Shell {
    pub fn new() -> Shell {
        Shell {
            history: Log::new(500)
        }
    }

    pub fn read_input(&mut self) -> String {
        print!("> ");
        let mut term = Term::stdout();
        io::stdout().flush().unwrap();
        let mut input = String::new();
        loop {
            let action = self.read_action();
            match action {
                Action::Submit => {
                    self.history.push(input.clone());
                    println!();
                    break;
                },
                Action::Back => {
                    self.history.back();
                    if let Some(value) = self.history.get() {
                        input = value.clone();
                    }
                },
                Action::Forward => {
                    self.history.forward();
                    if let Some(value) = self.history.get() {
                        input = value.clone();
                    }
                },
                Action::Char(value) => {
                    input.push(value);
                },
                Action::Backspace => {
                    input.pop();
                }
            }
            term.clear_line();
            term.write(("> ".to_owned() + &input).as_bytes());
        }

        input
    }

    fn read_action(&self) -> Action {
        let term = Term::stdout();
        loop {
            let key = term.read_key();
            if let Ok(key) = key {
                match key {
                    Key::ArrowUp => {
                        return Action::Back;
                    },
                    Key::ArrowDown => {
                        return Action::Forward;
                    },
                    Key::Enter => {
                        return Action::Submit;
                    },
                    Key::Char(value) => {
                        if value.is_backspace() {
                            return Action::Backspace;
                        } else if value.is_valid_char() {
                            return Action::Char(value);
                        }
                    },
                    _ => {}
                }
            }
        }
    }
}

struct Log {
    limit: usize,
    data: Vec<String>,
    pub index: usize
}

impl Log {
    pub fn new(limit: usize) -> Self {
        Self {
            limit,
            data: vec![String::new()],
            index: 0
        }
    }

    pub fn push(&mut self, value: String) {
        self.data.insert(1, value);
        self.index = 0;
        if self.data.len() > self.limit {
            self.data.pop();
        }
    }

    pub fn get(&self) -> Option<&String> {
        self.data.get(self.index)
    }

    pub fn back(&mut self) {
        if self.index + 1 < self.data.len() {
            self.index += 1;
        }
    }

    pub fn forward(&mut self) {
        self.index = self.index.saturating_sub(1);
    }
}

trait CharHelper {
    fn is_backspace(&self) -> bool;
    fn is_valid_char(&self) -> bool;
}

impl CharHelper for char {
    fn is_backspace(&self) -> bool {
        let mut buffer = [0u8; 4];
        self.encode_utf8(&mut buffer);

        self.is_ascii_control() && buffer[0] == 0x7F
    }

    fn is_valid_char(&self) -> bool {
        // TODO unicode pls
        let mut buffer = [0u8; 4];
        self.encode_utf8(&mut buffer);

        buffer[0] >= 0x20 && buffer[0] <= 0x7E
    }
}
