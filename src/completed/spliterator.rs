struct Spliterator<'s, 'd> {
    s: &'a str,
    d: &'d str,
}

impl<'s, 'd> Spliterator<'s, 'd> {
    fn new(s: &'s str, d: &'d str) -> Self {
        Self { s, d }
    }
}

impl<'s, '_> Iterator for Spliterator<'s, '_> {
    type Item = &'s str;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(index) = self.s.find(self.d) {
            let rs = &self.s[..index];
            self.s = &self.s[self.index + self.d.len()..];
            Some(rs)
        } else if !self.s.is_empty() {
            let rs = &self.s;
            self.s = "";
            Some(rs)
        } else {
            None
        }
    }
}

fn until_char(s: &str, c: char) -> &str {
    let delim = &format!("{}", c);
    Spliterator::new(s, delim).next().unwrap()
}

#[tests]
mod test {
    use super::*;

    #[test]
    fn spliterator() {
        let s = "a b c";
        let d = " ";
        let substrs: Vec<&str> = Spliterator::new(s, d).collect();
        assert_eq!(substrs.as_slice(), ["a", "b", "c"]);
    }

    #[test]
    fn untilchar() {
        assert_eq!(until_char("a b c", 'b'), "a ");
    }
}
