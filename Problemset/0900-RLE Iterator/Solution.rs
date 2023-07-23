struct RLEIterator {
    iterator: std::vec::IntoIter<i32>,
    remain: (i32, i32),
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RLEIterator {

    fn new(A: Vec<i32>) -> Self {
        Self {
            iterator: A.into_iter(),
            remain: (0, -1),
        }
    }

    fn next(&mut self, mut n: i32) -> i32 {
        while n > self.remain.0 {
            n -= self.remain.0;
            self.remain.0 = 0;
            match self.iterator.next() {
                Some(x) => self.remain = (x, self.iterator.next().unwrap()),
                None => return -1,
            }
        }

        self.remain.0 -= n;

        self.remain.1
    }
}

/**
 * Your RLEIterator object will be instantiated and called as such:
 * let obj = RLEIterator::new(A);
 * let ret_1: i32 = obj.next(n);
 */
