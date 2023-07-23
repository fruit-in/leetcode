# 1343. Number of Sub-arrays of Size K and Average Greater than or Equal to Threshold
Given an array of integers `arr` and two integers `k` and `threshold`.

Return *the number of sub-arrays* of size `k` and average greater than or equal to `threshold`.

#### Example 1:
<pre>
<strong>Input:</strong> arr = [2,2,2,2,5,5,5,8], k = 3, threshold = 4
<strong>Output:</strong> 3
<strong>Explanation:</strong> Sub-arrays [2,5,5],[5,5,5] and [5,5,8] have averages 4, 5 and 6 respectively. All other sub-arrays of size 3 have averages less than 4 (the threshold).
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> arr = [1,1,1,1,1], k = 1, threshold = 0
<strong>Output:</strong> 5
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> arr = [11,13,17,23,29,31,7,5,2,3], k = 3, threshold = 5
<strong>Output:</strong> 6
<strong>Explanation:</strong> The first 6 sub-arrays of size 3 have averages greater than 5. Note that averages are not integers.
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> arr = [7,7,7,7,7,7,7], k = 7, threshold = 7
<strong>Output:</strong> 1
</pre>

#### Example 5:
<pre>
<strong>Input:</strong> arr = [4,4,4,4], k = 4, threshold = 1
<strong>Output:</strong> 1
</pre>

#### Constraints:
* `1 <= arr.length <= 10^5`
* `1 <= arr[i] <= 10^4`
* `1 <= k <= arr.length`
* `0 <= threshold <= 10^4`

## Solutions (Ruby)

### 1. Sliding Window
```Ruby
# @param {Integer[]} arr
# @param {Integer} k
# @param {Integer} threshold
# @return {Integer}
def num_of_subarrays(arr, k, threshold)
  sum = arr[...k].sum
  i = 0
  ret = sum >= k * threshold ? 1 : 0

  (k...arr.size).each do |j|
    sum += arr[j] - arr[i]
    i += 1
    ret += 1 if sum >= k * threshold
  end

  ret
end
```

## Solutions (Rust)

### 1. Sliding Window
```Rust
impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>, k: i32, threshold: i32) -> i32 {
        let mut sum = arr[..k as usize].iter().sum::<i32>();
        let mut i = 0;
        let mut ret = (sum >= k * threshold) as i32;

        for j in k as usize..arr.len() {
            sum += arr[j] - arr[i];
            i += 1;
            if sum >= k * threshold {
                ret += 1;
            }
        }

        ret
    }
}
```
