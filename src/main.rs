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
                println!("{:?}", shell.current_prompt);
                //println!("{:?}", key_event);
                //println!("")
                let command = shell::Command::apply_key(&shell.current_prompt, key_event);
                shell.apply_command(command);

                let (prompt_str, cursor_pos) = shell.get_full_prompt_for_drawing();

                execute!(
                    stdout(),
                    MoveLeft(u16::MAX),
                    Clear(ClearType::CurrentLine),
                    Print(&prompt_str),
                    MoveLeft(u16::MAX),
                    MoveRight(cursor_pos as u16)
                )
                .unwrap();
            }
            _ => {}
        }
    }

    println!("Hello, world!");
}
