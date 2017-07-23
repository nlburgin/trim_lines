pub use std::str::Lines;

///A wrapper around Lines which trims the whitespace from each line.
pub struct TrimLines<'a>{
    l:Lines<'a>,
}

impl<'a> Iterator for TrimLines<'a> {
    type Item = &'a str;
    #[inline]
    fn next(&mut self) -> Option<&'a str> {
        let n = self.l.next();
        match n{
            Some(s) => Some(s.trim()),
            None => None,
        }
    }
}

impl<'a> DoubleEndedIterator for TrimLines<'a> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a str> {
        let n = self.l.next();
        return match n{
            Some(s) => Some(s.trim()),
            None => None,
        };
    }
}

impl<'a> TrimLines<'a>{
    ///Create a TrimLines from a Lines.
    #[inline]
    pub fn new(lines:Lines) -> TrimLines{
        return TrimLines{
            l:lines,
        };
    }
    
    ///Create a TrimLines directly from a string slice.
    #[inline]
    pub fn from_str(st:&str) -> TrimLines{
        return TrimLines{
            l:st.lines(),
        };
    }
}