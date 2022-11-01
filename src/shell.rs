use crate::history_completer::HistoryCompleter;

#[derive(Debug, Default)]
pub struct Shell {
    global_history: HistoryCompleter,
}

pub struct Zipper {
    text: String,
    pos: usize,
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
}
