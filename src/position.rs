use std::ops::Range;

#[derive(Debug, Clone, PartialEq)]
pub struct WithSpan<T> {
    pub(crate) _value: T,
    pub(crate) _line: usize,
    pub(crate) _span: Range<usize>,
}

impl<T> WithSpan<T> {
    #[inline]
    pub fn new(value: T, line: usize, span: Range<usize>) -> Self {
        Self {
            _value: value,
            _line: line,
            _span: span,
        }
    }

    #[inline]
    pub fn empty(value: T) -> Self {
        Self::new(value, 1, 0..0)
    }

    #[inline]
    pub fn value(self, value: T) -> Self {
        Self {
            _value: value,
            ..self
        }
    }

    #[inline]
    pub fn line(self, line: usize) -> Self {
        Self {
            _line: line,
            ..self
        }
    }

    #[inline]
    pub fn span(self, range: Range<usize>) -> Self {
        Self {
            _span: range,
            ..self
        }
    }

    #[inline]
    pub fn get_value(&self) -> &T {
        &self._value
    }

    #[inline]
    pub fn get_line(&self) -> usize {
        self._line
    }

    #[inline]
    pub fn get_span(&self) -> Range<usize> {
        self._span.clone()
    }
}

impl<T> AsRef<T> for WithSpan<T> {
    #[inline]
    fn as_ref(&self) -> &T {
        &self._value
    }
}

impl<T> From<(T, usize, Range<usize>)> for WithSpan<T> {
    #[inline]
    fn from((value, line, span): (T, usize, Range<usize>)) -> Self {
        Self {
            _value: value,
            _line: line,
            _span: span,
        }
    }
}
