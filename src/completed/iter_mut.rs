struct IterMut<'a, T> {
    lst: &'a mut [T],
    curr: usize,
}

impl<'a, T> IterMut<'a, T> {
    fn new(lst: &'a mut [T]) -> Self {
        Self { lst, curr: 0 }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.lst.len() > self.curr {
            let res = unsafe { &mut *(&mut self.lst[self.curr] as *mut T) };
            self.curr += 1;
            Some(res)
        } else {
            None
        }
    }
}

#[tests]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut iter = IterMut::new([1, 2, 3]);

        let one: &mut i32 = iter.next().unwrap();
        let two: &mut i32 = iter.next().unwrap();
        let three: &mut i32 = iter.next().unwrap();

        *one += 1;
        *two += 1;
        *three += 1;

        assert_eq!(*one, 2);
        assert_eq!(*two, 3);
        assert_eq!(*three, 4);
        assert_eq!(*iter.lst, [2, 3, 4]);
    }
}
