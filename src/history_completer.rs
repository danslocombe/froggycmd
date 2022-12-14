use crate::completer::{Completer, Completion};
use crate::trie::Trie;

#[derive(Debug, Default)]
pub struct HistoryCompleter {
    commands: Vec<Command>,
    trie: Trie,
}

#[derive(Debug)]
struct Command {
    //time: blah
    command: String,
}

impl Completer for HistoryCompleter {
    type CompletionIterator<'a> = HistoryCompletionIterator<'a>;
    fn complete<'a>(&'a self, prefix: &str) -> HistoryCompletionIterator<'a> {
        let trie_iter = self.trie.lookup(prefix.as_bytes());
        HistoryCompletionIterator { trie_iter }
    }
}

pub struct HistoryCompletionIterator<'a> {
    trie_iter: crate::trie::TrieNodeIterator<'a>,
}

impl<'a> Iterator for HistoryCompletionIterator<'a> {
    type Item = Completion;
    fn next(&mut self) -> Option<Completion> {
        let str = self.trie_iter.next()?;

        Some(Completion {
            text: unsafe { std::str::from_utf8_unchecked(str) }.to_owned(),
            score: 1.0,
        })
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_history_completer() {
        let mut trie = Trie::new();
        trie.insert(b"aabb", 0);
        trie.insert(b"aabc", 1);
        trie.insert(b"aacc", 2);

        //let completer = HistoryCompleter::new();
        //completer.insert()
        let completer = HistoryCompleter {
            commands: vec![],
            trie,
        };

        let mut completions = (&completer).complete("");
        let first = completions.next();
        assert_eq!("aabb", first.unwrap().text);
    }
}
