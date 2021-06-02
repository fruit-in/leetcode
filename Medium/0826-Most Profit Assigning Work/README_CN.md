# 826. 安排工作以达到最大收益
有一些工作：`difficulty[i]` 表示第 `i` 个工作的难度，`profit[i]` 表示第 `i` 个工作的收益。

现在我们有一些工人。`worker[i]` 是第 `i` 个工人的能力，即该工人只能完成难度小于等于 `worker[i]` 的工作。

每一个工人都最多只能安排一个工作，但是一个工作可以完成多次。

举个例子，如果 3 个工人都尝试完成一份报酬为 1 的同样工作，那么总收益为 $3。如果一个工人不能完成任何工作，他的收益为 $0 。

我们能得到的最大收益是多少？

#### 示例:
<pre>
<strong>输入:</strong> difficulty = [2,4,6,8,10], profit = [10,20,30,40,50], worker = [4,5,6,7]
<strong>输出:</strong> 100
<strong>解释:</strong> 工人被分配的工作难度是 [4,4,6,6] ，分别获得 [20,20,30,30] 的收益。
</pre>

#### 提示:
* `1 <= difficulty.length = profit.length <= 10000`
* `1 <= worker.length <= 10000`
* `difficulty[i], profit[i], worker[i]` 的范围是 `[1, 10^5]`

## 题解 (Rust)

### 1. 排序
```Rust
impl Solution {
    pub fn max_profit_assignment(difficulty: Vec<i32>, profit: Vec<i32>, worker: Vec<i32>) -> i32 {
        let mut work = difficulty
            .into_iter()
            .zip(profit.into_iter())
            .collect::<Vec<_>>();
        work.sort_unstable();
        let mut ret = 0;

        for i in 1..work.len() {
            work[i].1 = work[i].1.max(work[i - 1].1);
        }

        for w in worker {
            ret += match work.binary_search_by_key(&w, |&(d, _)| d) {
                Ok(i) => work[i].1,
                Err(0) => 0,
                Err(i) => work[i - 1].1,
            };
        }

        ret
    }
}
```
