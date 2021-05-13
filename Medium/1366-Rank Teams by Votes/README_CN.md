# 1366. 通过投票对团队排名
现在有一个特殊的排名系统，依据参赛团队在投票人心中的次序进行排名，每个投票者都需要按从高到低的顺序对参与排名的所有团队进行排位。

排名规则如下：
* 参赛团队的排名次序依照其所获「排位第一」的票的多少决定。如果存在多个团队并列的情况，将继续考虑其「排位第二」的票的数量。以此类推，直到不再存在并列的情况。
* 如果在考虑完所有投票情况后仍然出现并列现象，则根据团队字母的字母顺序进行排名。

给你一个字符串数组 `votes` 代表全体投票者给出的排位情况，请你根据上述排名规则对所有参赛团队进行排名。

请你返回能表示按排名系统 **排序后** 的所有团队排名的字符串。

#### 示例 1:
<pre>
<strong>输入:</strong> votes = ["ABC","ACB","ABC","ACB","ACB"]
<strong>输出:</strong> "ACB"
<strong>解释:</strong> A 队获得五票「排位第一」，没有其他队获得「排位第一」，所以 A 队排名第一。
B 队获得两票「排位第二」，三票「排位第三」。
C 队获得三票「排位第二」，两票「排位第三」。
由于 C 队「排位第二」的票数较多，所以 C 队排第二，B 队排第三。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> votes = ["WXYZ","XYZW"]
<strong>输出:</strong> "XWYZ"
<strong>解释:</strong> X 队在并列僵局打破后成为排名第一的团队。X 队和 W 队的「排位第一」票数一样，但是 X 队有一票「排位第二」，而 W 没有获得「排位第二」。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> votes = ["ZMNAGUEDSJYLBOPHRQICWFXTVK"]
<strong>输出:</strong> "ZMNAGUEDSJYLBOPHRQICWFXTVK"
<strong>解释:</strong> 只有一个投票者，所以排名完全按照他的意愿。
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> votes = ["BCA","CAB","CBA","ABC","ACB","BAC"]
<strong>输出:</strong> "ABC"
<strong>解释:</strong>
A 队获得两票「排位第一」，两票「排位第二」，两票「排位第三」。
B 队获得两票「排位第一」，两票「排位第二」，两票「排位第三」。
C 队获得两票「排位第一」，两票「排位第二」，两票「排位第三」。
完全并列，所以我们需要按照字母升序排名。
</pre>

#### 示例 5:
<pre>
<strong>输入:</strong> votes = ["M","M","M","M"]
<strong>输出:</strong> "M"
<strong>解释:</strong> 只有 M 队参赛，所以它排名第一。
</pre>

#### 提示:
* `1 <= votes.length <= 1000`
* `1 <= votes[i].length <= 26`
* `votes[i].length == votes[j].length` for `0 <= i, j < votes.length`.
* `votes[i][j]` 是英文 **大写** 字母
* `votes[i]` 中的所有字母都是唯一的
* `votes[0]` 中出现的所有字母 **同样也** 出现在 `votes[j]` 中，其中 `1 <= j < votes.length`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn rank_teams(votes: Vec<String>) -> String {
        let mut teams = votes[0].clone().into_bytes();
        let mut counter = [[0; 26]; 26];

        for vote in votes {
            for (r, c) in vote.bytes().enumerate() {
                counter[(c - b'A') as usize][r] -= 1;
            }
        }

        teams.sort_unstable_by_key(|&c| (counter[(c - b'A') as usize], c));

        String::from_utf8(teams).unwrap()
    }
}
```
