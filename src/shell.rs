use crate::history_completer::HistoryCompleter;

//use console::Key;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[derive(Debug, Default)]
pub struct Shell {
    global_history: HistoryCompleter,
    pub current_prompt: Zipper,
}

impl Shell {
    pub fn apply_command(&mut self, command: Command) {
        match command {
            Command::Text(z) => {
                self.current_prompt = z;
            }
            _ => {}
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Zipper {
    pub text: String,
    pub pos: usize,
}

impl Zipper {
    pub fn insert(&mut self, c: char) {
        self.text.insert(self.pos, c);
        self.pos += 1;
    }

    pub fn cursor_left(&mut self) {
        self.pos = self.pos.saturating_sub(1);
    }

    pub fn cursor_right(&mut self) {
        self.pos = (self.pos + 1).min(self.text.len() - 1);
    }

    pub fn delete_one(&mut self) {
        if (self.text.len() > 0) {
            self.cursor_left();
            _ = self.text.remove(self.pos);
        }
    }

    pub fn delete_all(&mut self) {
        self.text = String::new();
        self.pos = 0;
    }

    pub fn delete_word(&mut self) {
        todo!()
    }

    pub fn clone_insert(&self, c: char) -> Self {
        let mut new = self.clone();
        new.insert(c);
        new
    }
}

macro_rules! zipper_clone_apply {
    ($z:expr, $f:ident) => {{
        let mut new = $z.clone();
        new.$f();
        new
    }};
    ($z:expr, $f:ident, $a:expr) => {{
        let mut new = $z.clone();
        new.$f($a);
        new
    }};
}

pub enum Command {
    Text(Zipper),

    Run,
    Exit,
    Execute(String),
    Cls,

    Complete,
    PartialComplete,

    HistoryForward,
    HistoryBack,

    Search,

    NoOp,
}

impl Command {
    pub fn apply_key(current: &Zipper, k: KeyEvent) -> Self {
        match k.code {
            KeyCode::Left => Self::Text(zipper_clone_apply!(current, cursor_left)),
            KeyCode::Right => Self::Text(zipper_clone_apply!(current, cursor_right)),

            KeyCode::Backspace => {
                if k.modifiers.contains(KeyModifiers::CONTROL) {
                    Self::Text(zipper_clone_apply!(current, delete_word))
                } else {
                    Self::Text(zipper_clone_apply!(current, delete_one))
                }
            }

            KeyCode::Char(c) => Self::Text(zipper_clone_apply!(current, insert, c)),

            _ => Self::NoOp,
        }
    }
}
