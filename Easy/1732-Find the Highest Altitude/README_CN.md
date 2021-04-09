# 1732. 找到最高海拔
有一个自行车手打算进行一场公路骑行，这条路线总共由 `n + 1` 个不同海拔的点组成。自行车手从海拔为 `0` 的点 `0` 开始骑行。

给你一个长度为 `n` 的整数数组 `gain` ，其中 `gain[i]` 是点 `i` 和点 `i + 1` 的 **净海拔高度差**（`0 <= i < n`）。请你返回 **最高点的海拔** 。

#### 示例 1:
<pre>
<strong>输入:</strong> gain = [-5,1,5,0,-7]
<strong>输出:</strong> 1
<strong>解释:</strong> 海拔高度依次为 [0,-5,-4,1,1,-6] 。最高海拔为 1 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> gain = [-4,-3,-2,-1,4,3,2]
<strong>输出:</strong> 0
<strong>解释:</strong> 海拔高度依次为 [0,-4,-7,-9,-10,-6,-3,-1] 。最高海拔为 0 。
</pre>

#### 提示:
* `n == gain.length`
* `1 <= n <= 100`
* `-100 <= gain[i] <= 100`

## 题解 (Ruby)

### 1. 题解
```Ruby
# @param {Integer[]} gain
# @return {Integer}
def largest_altitude(gain)
  sum = 0
  ret = 0

  gain.each do |x|
    sum += x
    ret = sum if sum > ret
  end

  ret
end
```

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut ret = 0;

        for x in gain {
            sum += x;
            ret = ret.max(sum);
        }

        ret
    }
}
```
