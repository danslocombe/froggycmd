mod completer;
mod history_completer;
mod shell;
mod trie;

use console::Term;

fn main() {
    let shell = shell::Shell::default();

    let term = Term::stdout();

    while let Ok(k) = term.read_key() {
        println!("{:?}", k);
    }

    println!("Hello, world!");
}
