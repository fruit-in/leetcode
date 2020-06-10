# 78. Subsets
Given a set of **distinct** integers, *nums*, return all possible subsets (the power set).

**Note:** The solution set must not contain duplicate subsets.

#### Example:
<pre>
<strong>Input:</strong> nums = [1,2,3]
<strong>Output:</strong>
[
  [3],
  [1],
  [2],
  [1,2,3],
  [1,3],
  [2,3],
  [1,2],
  []
]
</pre>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ret = vec![vec![]];

        for num in nums {
            let mut temp = ret.clone();
            temp.iter_mut().for_each(|x| x.push(num));
            ret.append(&mut temp);
        }

        ret
    }
}
```
