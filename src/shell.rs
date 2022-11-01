use crate::history_completer::HistoryCompleter;
use console::Key;

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
    pub fn apply_key(current: &Zipper, k: console::Key) -> Self {
        match k {
            Key::ArrowLeft => Self::Text(zipper_clone_apply!(current, cursor_left)),
            Key::ArrowRight => Self::Text(zipper_clone_apply!(current, cursor_right)),
            Key::Char(c) => Self::Text(zipper_clone_apply!(current, insert, c)),

            _ => Self::NoOp,
        }
    }
}
