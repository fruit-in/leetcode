# 1488. Avoid Flood in The City
Your country has an infinite number of lakes. Initially, all the lakes are empty, but when it rains over the `nth` lake, the `nth` lake becomes full of water. If it rains over a lake which is **full of water**, there will be a **flood**. Your goal is to avoid the flood in any lake.

Given an integer array `rains` where:
* `rains[i] > 0` means there will be rains over the `rains[i]` lake.
* `rains[i] == 0` means there are no rains this day and you can choose **one lake** this day and **dry it**.

Return *an array `ans`* where:
* `ans.length == rains.length`
* `ans[i] == -1` if `rains[i] > 0`.
* `ans[i]` is the lake you choose to dry in the `ith` day if `rains[i] == 0`.

If there are multiple valid answers return **any** of them. If it is impossible to avoid flood return **an empty array**.

Notice that if you chose to dry a full lake, it becomes empty, but if you chose to dry an empty lake, nothing changes. (see example 4)

#### Example 1:
<pre>
<strong>Input:</strong> rains = [1,2,3,4]
<strong>Output:</strong> [-1,-1,-1,-1]
<strong>Explanation:</strong> After the first day full lakes are [1]
After the second day full lakes are [1,2]
After the third day full lakes are [1,2,3]
After the fourth day full lakes are [1,2,3,4]
There's no day to dry any lake and there is no flood in any lake.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> rains = [1,2,0,0,2,1]
<strong>Output:</strong> [-1,-1,2,1,-1,-1]
<strong>Explanation:</strong> After the first day full lakes are [1]
After the second day full lakes are [1,2]
After the third day, we dry lake 2. Full lakes are [1]
After the fourth day, we dry lake 1. There is no full lakes.
After the fifth day, full lakes are [2].
After the sixth day, full lakes are [1,2].
It is easy that this scenario is flood-free. [-1,-1,1,2,-1,-1] is another acceptable scenario.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> rains = [1,2,0,1,2]
<strong>Output:</strong> []
<strong>Explanation:</strong> After the second day, full lakes are  [1,2]. We have to dry one lake in the third day.
After that, it will rain over lakes [1,2]. It's easy to prove that no matter which lake you choose to dry in the 3rd day, the other one will flood.
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> rains = [69,0,0,0,69]
<strong>Output:</strong> [-1,69,1,1,-1]
<strong>Explanation:</strong> Any solution on one of the forms [-1,69,x,y,-1], [-1,x,69,y,-1] or [-1,x,y,69,-1] is acceptable where 1 <= x,y <= 10^9
</pre>

#### Example 5:
<pre>
<strong>Input:</strong> rains = [10,20,20]
<strong>Output:</strong> []
<strong>Explanation:</strong> It will rain over lake 20 two consecutive days. There is no chance to dry any lake.
</pre>

#### Constraints:
* <code>1 <= rains.length <= 10<sup>5</sup></code>
* <code>0 <= rains[i] <= 10<sup>9</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn avoid_flood(rains: Vec<i32>) -> Vec<i32> {
        let mut dry_days = vec![];
        let mut rain_days = HashMap::new();
        let mut ans = vec![-1; rains.len()];

        for i in 0..rains.len() {
            if rains[i] == 0 {
                dry_days.push(i);
            } else if let Some(j) = rain_days.insert(rains[i], i) {
                match dry_days.binary_search(&j) {
                    Err(k) if k == dry_days.len() => return vec![],
                    Err(k) => {
                        ans[dry_days[k]] = rains[i];
                        dry_days.remove(k);
                    }
                    _ => (),
                }
            }
        }

        for i in dry_days {
            ans[i] = 1;
        }

        ans
    }
}
```
