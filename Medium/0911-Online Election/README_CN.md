# 911. 在线选举
在选举中，第 `i` 张票是在时间为 `times[i]` 时投给 `persons[i]` 的。

现在，我们想要实现下面的查询函数： `TopVotedCandidate.q(int t)` 将返回在 `t` 时刻主导选举的候选人的编号。

在 `t` 时刻投出的选票也将被计入我们的查询之中。在平局的情况下，最近获得投票的候选人将会获胜。

#### 示例:
<pre>
<strong>输入:</strong> ["TopVotedCandidate","q","q","q","q","q","q"], [[[0,1,1,0,0,1,0],[0,5,10,15,20,25,30]],[3],[12],[25],[15],[24],[8]]
<strong>输出:</strong> [null,0,1,1,0,0,1]
<strong>解释:</strong>
时间为 3，票数分布情况是 [0]，编号为 0 的候选人领先。
时间为 12，票数分布情况是 [0,1,1]，编号为 1 的候选人领先。
时间为 25，票数分布情况是 [0,1,1,0,0,1]，编号为 1 的候选人领先（因为最近的投票结果是平局）。
在时间 15、24 和 8 处继续执行 3 个查询。
</pre>

#### 提示:
1. `1 <= persons.length = times.length <= 5000`
2. `0 <= persons[i] <= persons.length`
3. `times` 是严格递增的数组，所有元素都在 `[0, 10^9]` 范围中。
4. 每个测试用例最多调用 `10000` 次 `TopVotedCandidate.q`。
5. `TopVotedCandidate.q(int t)` 被调用时总是满足 `t >= times[0]`。

## 题解 (Rust)

### 1. 二分查找
```Rust
use std::collections::HashMap;

struct TopVotedCandidate {
    winners: Vec<(i32, i32)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TopVotedCandidate {
    fn new(persons: Vec<i32>, times: Vec<i32>) -> Self {
        let mut counter = HashMap::new();
        let mut max = 0;
        let mut winner = 0;
        let mut winners = vec![];

        for (p, t) in persons.into_iter().zip(times.into_iter()) {
            let c = counter.entry(p).or_insert(0);
            *c += 1;
            if *c >= max {
                max = *c;
                winner = p;
            }
            winners.push((t, winner));
        }

        Self { winners }
    }

    fn q(&self, t: i32) -> i32 {
        match self.winners.binary_search_by_key(&t, |&(time, _)| time) {
            Ok(i) => self.winners[i].1,
            Err(i) => self.winners[i - 1].1,
        }
    }
}

/**
 * Your TopVotedCandidate object will be instantiated and called as such:
 * let obj = TopVotedCandidate::new(persons, times);
 * let ret_1: i32 = obj.q(t);
 */
```
