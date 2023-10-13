// #[derive(Debug, PartialEq, Eq)]
// pub enum NestedInteger {
//   Int(i32),
//   List(Vec<NestedInteger>)
// }
use crate::NestedInteger::Int;
use crate::NestedInteger::List;

struct NestedIterator {
    index: usize,
    nums: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NestedIterator {
    fn new(nestedList: Vec<NestedInteger>) -> Self {
        let mut nums = vec![];

        for elem in nestedList {
            match elem {
                Int(x) => nums.push(x),
                List(list) => {
                    let mut list = Self::new(list);

                    while list.has_next() {
                        nums.push(list.next());
                    }
                }
            }
        }

        Self { index: 0, nums }
    }

    fn next(&mut self) -> i32 {
        self.index += 1;

        self.nums[self.index - 1]
    }

    fn has_next(&self) -> bool {
        self.index < self.nums.len()
    }
}

/**
 * Your NestedIterator object will be instantiated and called as such:
 * let obj = NestedIterator::new(nestedList);
 * let ret_1: i32 = obj.next();
 * let ret_2: bool = obj.has_next();
 */
