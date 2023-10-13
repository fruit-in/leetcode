# 341. 扁平化嵌套列表迭代器
给你一个嵌套的整数列表 `nestedList` 。每个元素要么是一个整数，要么是一个列表；该列表的元素也可能是整数或者是其他列表。请你实现一个迭代器将其扁平化，使之能够遍历这个列表中的所有整数。

实现扁平迭代器类 `NestedIterator` ：

* `NestedIterator(List<NestedInteger> nestedList)` 用嵌套列表 `nestedList` 初始化迭代器。
* `int next()` 返回嵌套列表的下一个整数。
* `boolean hasNext()` 如果仍然存在待迭代的整数，返回 `true` ；否则，返回 `false` 。

你的代码将会用下述伪代码检测：

```
initialize iterator with nestedList
res = []
while iterator.hasNext()
    append iterator.next() to the end of res
return res
```

如果 `res` 与预期的扁平化列表匹配，那么你的代码将会被判为正确。

#### 示例 1:
<pre>
<strong>输入:</strong> nestedList = [[1,1],2,[1,1]]
<strong>输出:</strong> [1,1,2,1,1]
<strong>Explanation:</strong> 通过重复调用 next 直到 hasNext 返回 false，next 返回的元素的顺序应该是: [1,1,2,1,1]。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nestedList = [1,[4,[6]]]
<strong>输出:</strong> [1,4,6]
<strong>解释:</strong> 通过重复调用 next 直到 hasNext 返回 false，next 返回的元素的顺序应该是: [1,4,6]。
</pre>

#### 提示:
* `1 <= nestedList.length <= 500`
* 嵌套列表中的整数值在范围 <code>[-10<sup>6</sup>, 10<sup>6</sup>]</code> 内

## 题解 (Rust)

### 1. 题解
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
