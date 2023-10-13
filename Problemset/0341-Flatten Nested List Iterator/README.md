# 341. Flatten Nested List Iterator
You are given a nested list of integers `nestedList`. Each element is either an integer or a list whose elements may also be integers or other lists. Implement an iterator to flatten it.

Implement the `NestedIterator` class:

* `NestedIterator(List<NestedInteger> nestedList)` Initializes the iterator with the nested list `nestedList`.
* `int next()` Returns the next integer in the nested list.
* `boolean hasNext()` Returns `true` if there are still some integers in the nested list and `false` otherwise.

Your code will be tested with the following pseudocode:

```
initialize iterator with nestedList
res = []
while iterator.hasNext()
    append iterator.next() to the end of res
return res
```

If `res` matches the expected flattened list, then your code will be judged as correct.

#### Example 1:
<pre>
<strong>Input:</strong> nestedList = [[1,1],2,[1,1]]
<strong>Output:</strong> [1,1,2,1,1]
<strong>Explanation:</strong> By calling next repeatedly until hasNext returns false, the order of elements returned by next should be: [1,1,2,1,1].
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nestedList = [1,[4,[6]]]
<strong>Output:</strong> [1,4,6]
<strong>Explanation:</strong> By calling next repeatedly until hasNext returns false, the order of elements returned by next should be: [1,4,6].
</pre>

#### Constraints:
* `1 <= nestedList.length <= 500`
* The values of the integers in the nested list is in the range <code>[-10<sup>6</sup>, 10<sup>6</sup>]</code>.

## Solutions (Rust)

### 1. Solution
```Rust
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
```
