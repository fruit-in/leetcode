# 2251. Number of Flowers in Full Bloom
You are given a **0-indexed** 2D integer array `flowers`, where <code>flowers[i] = [start<sub>i</sub>, end<sub>i</sub>]</code> means the <code>i<sup>th</sup></code> flower will be in **full bloom** from <code>start<sub>i</sub></code> to <code>end<sub>i</sub></code> (**inclusive**). You are also given a **0-indexed** integer array `people` of size `n`, where `people[i]` is the time that the <code>i<sup>th</sup></code> person will arrive to see the flowers.

Return *an integer array* `answer` *of size* `n`, *where* `answer[i]` *is the **number** of flowers that are in full bloom when the* <code>i<sup>th</sup></code> *person arrives*.

#### Example 1:
![](https://assets.leetcode.com/uploads/2022/03/02/ex1new.jpg)
<pre>
<strong>Input:</strong> flowers = [[1,6],[3,7],[9,12],[4,13]], poeple = [2,3,7,11]
<strong>Output:</strong> [1,2,2,2]
<strong>Explanation:</strong> The figure above shows the times when the flowers are in full bloom and when the people arrive.
For each person, we return the number of flowers in full bloom during their arrival.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2022/03/02/ex2new.jpg)
<pre>
<strong>Input:</strong> flowers = [[1,10],[3,3]], poeple = [3,3,2]
<strong>Output:</strong> [2,2,1]
<strong>Explanation:</strong> The figure above shows the times when the flowers are in full bloom and when the people arrive.
For each person, we return the number of flowers in full bloom during their arrival.
</pre>

#### Constraints:
* <code>1 <= flowers.length <= 5 * 10<sup>4</sup></code>
* `flowers[i].length == 2`
* <code>1 <= start<sub>i</sub> <= end<sub>i</sub> <= 10<sup>9</sup></code>
* <code>1 <= people.length <= 5 * 10<sup>4</sup></code>
* <code>1 <= people[i] <= 10<sup>9</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn full_bloom_flowers(flowers: Vec<Vec<i32>>, people: Vec<i32>) -> Vec<i32> {
        let mut flowers = flowers;
        let mut people = people
            .into_iter()
            .enumerate()
            .map(|(i, p)| (p, i))
            .collect::<Vec<_>>();
        let mut flowers_heap = BinaryHeap::new();
        let mut i = 0;
        let mut answer = vec![0; people.len()];

        flowers.sort_unstable();
        people.sort_unstable();

        for &(p, j) in &people {
            while let Some(&Reverse(end)) = flowers_heap.peek() {
                if end >= p {
                    break;
                }

                flowers_heap.pop();
            }

            while i < flowers.len() && flowers[i][0] <= p {
                if flowers[i][1] >= p {
                    flowers_heap.push(Reverse(flowers[i][1]));
                }

                i += 1;
            }

            answer[j] = flowers_heap.len() as i32;
        }

        answer
    }
}
```
