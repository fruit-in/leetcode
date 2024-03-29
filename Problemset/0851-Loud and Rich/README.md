# 851. Loud and Rich
There is a group of `n` people labeled from `0` to `n - 1` where each person has a different amount of money and a different level of quietness.

You are given an array `richer` where <code>richer[i] = [a<sub>i</sub>, b<sub>i</sub>]</code> indicates that <code>a<sub>i</sub></code> has more money than <code>b<sub>i</sub></code> and an integer array `quiet` where `quiet[i]` is the quietness of the <code>i<sup>th</sup></code> person. All the given data in richer are **logically correct** (i.e., the data will not lead you to a situation where `x` is richer than `y` and `y` is richer than `x` at the same time).

Return *an integer array* `answer` *where* `answer[x] = y` *if* `y` *is the least quiet person (that is, the person* `y` *with the smallest value of* `quiet[y]`*) among all people who definitely have equal to or more money than the person* `x`.

#### Example 1:
<pre>
<strong>Input:</strong> richer = [[1,0],[2,1],[3,1],[3,7],[4,3],[5,3],[6,3]], quiet = [3,2,5,4,6,1,7,0]
<strong>Output:</strong> [5,5,2,5,4,5,6,7]
<strong>Explanation:</strong>
answer[0] = 5.
Person 5 has more money than 3, which has more money than 1, which has more money than 0.
The only person who is quieter (has lower quiet[x]) is person 7, but it is not clear if they have more money than person 0.
answer[7] = 7.
Among all people that definitely have equal to or more money than person 7 (which could be persons 3, 4, 5, 6, or 7), the person who is the quietest (has lower quiet[x]) is person 7.
The other answers can be filled out with similar reasoning.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> richer = [], quiet = [0]
<strong>Output:</strong> [0]
</pre>

#### Constraints:
* `n == quiet.length`
* `1 <= n <= 500`
* `0 <= quiet[i] < n`
* All the values of `quiet` are **unique**.
* `0 <= richer.length <= n * (n - 1) / 2`
* <code>0 <= a<sub>i</sub>, b<sub>i</sub> < n</code>
* <code>a<sub>i</sub> != b<sub>i</sub></code>
* All the pairs of `richer` are **unique**.
* The observations in `richer` are all logically consistent.

## Solutions (Rust)

### 1. Solution
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
