#![deprecated(since="0.2.0", note="please use `map(str::trim)` instead")]
pub use std::str::Lines;

///A wrapper around Lines which trims the whitespace from each line.
pub struct TrimLines<'a> {
    l: Lines<'a>,
}

impl<'a> Iterator for TrimLines<'a> {
    type Item = &'a str;
    #[inline]
    fn next(&mut self) -> Option<&'a str> {
        let n = self.l.next();
        match n {
            Some(s) => Some(s.trim()),
            None => None,
        }
    }
}

impl<'a> DoubleEndedIterator for TrimLines<'a> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a str> {
        let n = self.l.next_back();
        return match n {
            Some(s) => Some(s.trim()),
            None => None,
        };
    }
}

impl<'a> TrimLines<'a> {
    ///Create a TrimLines from a Lines.
    #[inline]
    pub fn new(lines: Lines) -> TrimLines {
        return TrimLines { l: lines };
    }

    ///Create a TrimLines directly from a string slice.
    #[inline]
    pub fn from_str(st: &str) -> TrimLines {
        return TrimLines { l: st.lines() };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forward_iter() {
        let text = "    foo    \r\n    bar    \n   \n    baz    \n";
        let mut lines = TrimLines::new(text.lines());

        assert_eq!(Some("foo"), lines.next());
        assert_eq!(Some("bar"), lines.next());
        assert_eq!(Some(""), lines.next());
        assert_eq!(Some("baz"), lines.next());
        assert_eq!(None, lines.next());
    }

    #[test]
    fn backward_iter() {
        let text = "    foo    \r\n    bar    \n   \n    baz    \n";
        let mut lines = TrimLines::new(text.lines());

        assert_eq!(Some("baz"), lines.next_back());
        assert_eq!(Some(""), lines.next_back());
        assert_eq!(Some("bar"), lines.next_back());
        assert_eq!(Some("foo"), lines.next_back());
        assert_eq!(None, lines.next_back());
    }
}
