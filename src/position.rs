use std::ops::Range;

#[derive(Debug, PartialEq)]
pub struct WithSpan<T> {
    pub value: T,
    pub span: Range<usize>,
}

impl<T> WithSpan<T> {
    pub fn new(value: T, span: Range<usize>) -> Self {
        Self { value, span }
    }
}

impl<T> AsRef<T> for WithSpan<T> {
    fn as_ref(&self) -> &T {
        &self.value
    }
}
