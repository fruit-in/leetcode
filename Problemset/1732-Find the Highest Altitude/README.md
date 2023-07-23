# 1732. Find the Highest Altitude
There is a biker going on a road trip. The road trip consists of `n + 1` points at different altitudes. The biker starts his trip on point `0` with altitude equal `0`.

You are given an integer array `gain` of length `n` where `gain[i]` is the **net gain in altitude** between points `i` and `i + 1` for all (`0 <= i < n`). Return *the **highest altitude** of a point*.

#### Example 1:
<pre>
<strong>Input:</strong> gain = [-5,1,5,0,-7]
<strong>Output:</strong> 1
<strong>Explanation:</strong> The altitudes are [0,-5,-4,1,1,-6]. The highest is 1.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> gain = [-4,-3,-2,-1,4,3,2]
<strong>Output:</strong> 0
<strong>Explanation:</strong> The altitudes are [0,-4,-7,-9,-10,-6,-3,-1]. The highest is 0.
</pre>

#### Constraints:
* `n == gain.length`
* `1 <= n <= 100`
* `-100 <= gain[i] <= 100`

## Solutions (Ruby)

### 1. Solution
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

## Solutions (Rust)

### 1. Solution
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
