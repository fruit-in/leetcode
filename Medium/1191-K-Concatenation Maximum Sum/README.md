# 1191. K-Concatenation Maximum Sum
Given an integer array `arr` and an integer `k`, modify the array by repeating it `k` times.

For example, if `arr = [1, 2]` and `k = 3` then the modified array will be `[1, 2, 1, 2, 1, 2]`.

Return the maximum sub-array sum in the modified array. Note that the length of the sub-array can be `0` and its sum in that case is `0`.

As the answer can be very large, return the answer **modulo** <code>10<sup>9</sup> + 7</code>.

#### Example 1:
<pre>
<strong>Input:</strong> arr = [1,2], k = 3
<strong>Output:</strong> 9
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> arr = [1,-2,1], k = 5
<strong>Output:</strong> 2
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> arr = [-1,-2], k = 7
<strong>Output:</strong> 0
</pre>

#### Constraints:
* <code>1 <= arr.length <= 10<sup>5</sup></code>
* <code>1 <= k <= 10<sup>5</sup></code>
* <code>-10<sup>4</sup> <= arr[i] <= 10<sup>4</sup></code>

## Solutions (Ruby)

### 1. Solution
```Ruby
# @param {Integer[]} arr
# @param {Integer} k
# @return {Integer}
def k_concatenation_max_sum(arr, k)
  l_sum = 0
  r_sum = 0
  l_max_sum = 0
  r_max_sum = 0
  l_min_sum = 0
  ret = 0

  (1..arr.size).each do |i|
    l_sum += arr[i - 1]
    r_sum += arr[-i]
    l_max_sum = [l_max_sum, l_sum].max
    r_max_sum = [r_max_sum, r_sum].max
    l_min_sum = [l_min_sum, l_sum].min
    ret = [ret, l_sum - l_min_sum].max
  end

  k == 1 ? ret : [ret, [l_sum, 0].max * (k - 2) + l_max_sum + r_max_sum].max % 1_000_000_007
end
```

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn k_concatenation_max_sum(arr: Vec<i32>, k: i32) -> i32 {
        let mut l_sum = 0i64;
        let mut r_sum = 0i64;
        let mut l_max_sum = 0i64;
        let mut r_max_sum = 0i64;
        let mut l_min_sum = 0i64;
        let mut ret = 0i64;

        for i in 0..arr.len() {
            l_sum += arr[i] as i64;
            r_sum += arr[arr.len() - 1 - i] as i64;
            l_max_sum = l_max_sum.max(l_sum);
            r_max_sum = r_max_sum.max(r_sum);
            l_min_sum = l_min_sum.min(l_sum);
            ret = ret.max(l_sum - l_min_sum);
        }

        match k {
            1 => ret as i32,
            _ => {
                (ret.max(l_sum.max(0) * (k as i64 - 2) + l_max_sum + r_max_sum) % 1_000_000_007)
                    as i32
            }
        }
    }
}
```
