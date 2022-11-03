pub struct Completion {
    pub text: String,
    pub score: f32,
}

pub trait Completer {
    // Give space for completion iteartors to reference the underlying completer
    type CompletionIterator<'a>: Iterator<Item = Completion>
    where
        Self: 'a;

    fn complete<'a>(&'a self, prefix: &str) -> Self::CompletionIterator<'a>;
}
