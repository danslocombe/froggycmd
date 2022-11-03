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

    fn preprompt(&self) -> String {
        format!("{}>>>", "some/dir")
    }

    pub fn get_full_prompt_for_drawing(&self) -> (String, usize) {
        let preprompt = self.preprompt();
        (
            format!("{}{}", preprompt, self.current_prompt.text),
            self.current_prompt.pos + preprompt.len(),
        )
    }
}

#[derive(Debug, Default, Clone)]
pub struct Zipper {
    pub text: String,
    pub pos: usize,
}

fn is_block_delimiter(c: char) -> bool {
    match c {
        ' ' | '/' | '\\' => true,
        _ => false,
    }
}

fn delete_between(x: &str, start: usize, end: usize) -> String {
    // TODO could write this as taking &mut String and not reallocate
    // Doesnt really matter.

    assert!(start < end);
    assert!(end < x.len());

    format!("{}{}", &x[0..start], &x[end..])
}

impl Zipper {
    fn current(&self) -> Option<char> {
        if self.pos < self.text.len() - 1 {
            self.text.chars().nth(self.pos)
        } else {
            None
        }
    }

    pub fn insert(&mut self, c: char) {
        self.text.insert(self.pos, c);
        self.pos += 1;
    }

    pub fn cursor_left(&mut self) {
        self.pos = self.pos.saturating_sub(1);
    }

    pub fn cursor_right(&mut self) {
        self.pos = (self.pos + 1).min(self.text.len());
    }

    pub fn cursor_block_left(&mut self) {
        loop {
            if (self.pos == 0) {
                return;
            }

            if let Some(x) = self.current() {
                if (is_block_delimiter(x)) {
                    return;
                }
            }

            self.cursor_left();
        }
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
        let start_pos = self.pos;
        self.cursor_block_left();

        self.text = if (start_pos >= self.text.len()) {
            self.text[0..self.pos].to_owned()
        } else {
            delete_between(&self.text, self.pos, start_pos.min(self.text.len() - 1))
        };
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
            KeyCode::Left => {
                if k.modifiers.contains(KeyModifiers::CONTROL) {
                    Self::Text(zipper_clone_apply!(current, cursor_block_left))
                } else {
                    Self::Text(zipper_clone_apply!(current, cursor_left))
                }
            }
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
