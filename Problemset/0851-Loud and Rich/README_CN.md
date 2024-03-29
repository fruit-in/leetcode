# 851. 喧闹和富有
有一组 `n` 个人作为实验对象，从 `0` 到 `n - 1` 编号，其中每个人都有不同数目的钱，以及不同程度的安静值（quietness）。为了方便起见，我们将编号为 `x` 的人简称为 "person `x` "。

给你一个数组 `richer` ，其中 <code>richer[i] = [a<sub>i</sub>, b<sub>i</sub>]</code> 表示 person <code>a<sub>i</sub></code> 比 person <code>b<sub>i</sub></code> 更有钱。另给你一个整数数组 `quiet` ，其中 `quiet[i]` 是 person `i` 的安静值。`richer` 中所给出的数据 逻辑自洽（也就是说，在 person `x` 比 person `y` 更有钱的同时，不会出现 person `y` 比 person `x` 更有钱的情况 ）。

现在，返回一个整数数组 `answer` 作为答案，其中 `answer[x] = y` 的前提是，在所有拥有的钱肯定不少于 person `x` 的人中，person `y` 是最不安静的人（也就是安静值 `quiet[y]` 最小的人）。

#### 示例 1:
<pre>
<strong>输入:</strong> richer = [[1,0],[2,1],[3,1],[3,7],[4,3],[5,3],[6,3]], quiet = [3,2,5,4,6,1,7,0]
<strong>输出:</strong> [5,5,2,5,4,5,6,7]
<strong>解释:</strong>
answer[0] = 5，
person 5 比 person 3 有更多的钱，person 3 比 person 1 有更多的钱，person 1 比 person 0 有更多的钱。
唯一较为安静（有较低的安静值 quiet[x]）的人是 person 7，
但是目前还不清楚他是否比 person 0 更有钱。
answer[7] = 7，
在所有拥有的钱肯定不少于 person 7 的人中（这可能包括 person 3，4，5，6 以及 7），
最安静（有较低安静值 quiet[x]）的人是 person 7。
其他的答案也可以用类似的推理来解释。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> richer = [], quiet = [0]
<strong>输出:</strong> [0]
</pre>

#### 提示:
* `n == quiet.length`
* `1 <= n <= 500`
* `0 <= quiet[i] < n`
* `quiet` 的所有值 **互不相同**
* `0 <= richer.length <= n * (n - 1) / 2`
* <code>0 <= a<sub>i</sub>, b<sub>i</sub> < n</code>
* <code>a<sub>i</sub> != b<sub>i</sub></code>
* `richer` 中的所有数对 **互不相同**
* 对 `richer` 的观察在逻辑上是一致的

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn loud_and_rich(richer: Vec<Vec<i32>>, quiet: Vec<i32>) -> Vec<i32> {
        let n = quiet.len();
        let mut richer_count = vec![0; n];
        let mut poorer_people = vec![vec![]; n];
        let mut people = vec![];
        let mut answer = (0..n as i32).collect::<Vec<_>>();

        for pair in &richer {
            richer_count[pair[1] as usize] += 1;
            poorer_people[pair[0] as usize].push(pair[1] as usize);
        }

        for i in 0..n {
            if richer_count[i] == 0 {
                people.push(i);
            }
        }

        while let Some(x) = people.pop() {
            for &y in &poorer_people[x] {
                richer_count[y] -= 1;
                if richer_count[y] == 0 {
                    people.push(y);
                }
                if quiet[answer[x] as usize] < quiet[answer[y] as usize] {
                    answer[y] = answer[x];
                }
            }
        }

        answer
    }
}
```
