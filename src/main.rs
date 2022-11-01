mod completer;
mod history_completer;
mod shell;
mod trie;

use crossterm::event::{
    self, KeyboardEnhancementFlags, PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags,
};
use crossterm::ExecutableCommand;

use crossterm::cursor::{MoveLeft, MoveRight};
use crossterm::execute;
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType};

use std::io::{stdout, Write};

fn main() {
    let mut shell = shell::Shell::default();

    while let Ok(event) = event::read() {
        match (event) {
            event::Event::Key(key_event) => {
                println!("{:?}", key_event);
                let command = shell::Command::apply_key(&shell.current_prompt, key_event);
                shell.apply_command(command);

                execute!(
                    stdout(),
                    Clear(ClearType::CurrentLine),
                    Print(&shell.current_prompt.text),
                    MoveLeft(u16::MAX),
                    MoveRight(shell.current_prompt.pos as u16)
                )
                .unwrap();
            }
            _ => {}
        }
    }

    println!("Hello, world!");
}
