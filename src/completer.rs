pub struct Completion {
    pub text: String,
    pub score: f32,
}

pub trait Completer {
    type ConcreteCompletionIterator: Iterator<Item = Completion>;
    fn complete(&self, prefix: &str) -> Box<Self::ConcreteCompletionIterator>;
}
