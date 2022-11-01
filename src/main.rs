mod completer;
mod history_completer;
mod shell;
mod trie;

use console::Term;

use std::io::Write;

fn main() {
    let mut shell = shell::Shell::default();

    let mut term = Term::stdout();

    while let Ok(k) = term.read_key() {
        //println!("{:?}", k);
        let command = shell::Command::apply_key(&shell.current_prompt, k);
        shell.apply_command(command);

        _ = term.clear_line().unwrap();
        term.write(&shell.current_prompt.text.as_bytes()).unwrap();

        _ = term.move_cursor_left(usize::MAX).unwrap();
        _ = term.move_cursor_right(shell.current_prompt.pos).unwrap();
    }

    println!("Hello, world!");
}
