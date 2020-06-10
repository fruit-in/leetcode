# 78. 子集
给定一组**不含重复元素**的整数数组 *nums*，返回该数组所有可能的子集（幂集）。

**说明:** 解集不能包含重复的子集。

#### 示例:
<pre>
<strong>输入:</strong> nums = [1,2,3]
<strong>输出:</strong>
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

## 题解 (Rust)

### 1. 题解
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
