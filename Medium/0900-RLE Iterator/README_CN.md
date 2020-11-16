# 900. RLE 迭代器
编写一个遍历游程编码序列的迭代器。

迭代器由 `RLEIterator(int[] A)` 初始化，其中 `A` 是某个序列的游程编码。更具体地，对于所有偶数 `i`，`A[i]` 告诉我们在序列中重复非负整数值 `A[i + 1]` 的次数。

迭代器支持一个函数：`next(int n)`，它耗尽接下来的  `n` 个元素（`n >= 1`）并返回以这种方式耗去的最后一个元素。如果没有剩余的元素可供耗尽，则  `next` 返回 `-1` 。

例如，我们以 `A = [3,8,0,9,2,5]` 开始，这是序列 `[8,8,8,5,5]` 的游程编码。这是因为该序列可以读作 “三个八，零个九，两个五”。

#### 示例:
<pre>
<b>输入:</b> ["RLEIterator","next","next","next","next"], [[[3,8,0,9,2,5]],[2],[1],[1],[2]]
<b>输出:</b> [null,8,8,5,-1]
<b>解释:</b>
RLEIterator 由 RLEIterator([3,8,0,9,2,5]) 初始化。
这映射到序列 [8,8,8,5,5]。
然后调用 RLEIterator.next 4次。

.next(2) 耗去序列的 2 个项，返回 8。现在剩下的序列是 [8, 5, 5]。

.next(1) 耗去序列的 1 个项，返回 8。现在剩下的序列是 [5, 5]。

.next(1) 耗去序列的 1 个项，返回 5。现在剩下的序列是 [5]。

.next(2) 耗去序列的 2 个项，返回 -1。 这是由于第一个被耗去的项是 5，
但第二个项并不存在。由于最后一个要耗去的项不存在，我们返回 -1。
</pre>

#### 提示:
1. `0 <= A.length <= 1000`
2. `A.length` 是偶数。
3. `0 <= A[i] <= 10^9`
4. 每个测试用例最多调用 `1000` 次 `RLEIterator.next(int n)`。
5. 每次调用 `RLEIterator.next(int n)` 都有 `1 <= n <= 10^9` 。

## 题解 (Rust)

### 1. 题解
```Rust
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
```
