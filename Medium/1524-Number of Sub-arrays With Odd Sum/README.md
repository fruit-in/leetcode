# 1524. Number of Sub-arrays With Odd Sum
Given an array of integers `arr`. Return *the number of sub-arrays* with **odd** sum.

As the answer may grow large, the answer **must be** computed modulo `10^9 + 7`.

#### Example 1:
<pre>
<strong>Input:</strong> arr = [1,3,5]
<strong>Output:</strong> 4
<strong>Explanation:</strong> All sub-arrays are [[1],[1,3],[1,3,5],[3],[3,5],[5]]
All sub-arrays sum are [1,4,9,3,8,5].
Odd sums are [1,9,3,5] so the answer is 4.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> arr = [2,4,6]
<strong>Output:</strong> 0
<strong>Explanation:</strong> All sub-arrays are [[2],[2,4],[2,4,6],[4],[4,6],[6]]
All sub-arrays sum are [2,6,12,4,10,6].
All sub-arrays have even sum and the answer is 0.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> arr = [1,2,3,4,5,6,7]
<strong>Output:</strong> 16
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> arr = [100,100,99,99]
<strong>Output:</strong> 4
</pre>

#### Example 5:
<pre>
<strong>Input:</strong> arr = [7]
<strong>Output:</strong> 1
</pre>

#### Constraints:
* `1 <= arr.length <= 10^5`
* `1 <= arr[i] <= 100`

## Solutions (Ruby)

### 1. Prefix Sum
```Ruby
# @param {Integer[]} arr
# @return {Integer}
def num_of_subarrays(arr)
  sum = 0
  odd = 0
  even = 1
  ret = 0

  arr.each do |x|
    sum += x
    if sum.odd?
      odd += 1
      ret += even
    else
      even += 1
      ret += odd
    end
  end

  ret % 1_000_000_007
end
```

## Solutions (Rust)

### 1. Prefix Sum
```Rust
impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut odd = 0;
        let mut even = 1;
        let mut ret = 0;

        for x in arr {
            sum += x;
            if sum % 2 == 1 {
                odd += 1;
                ret = (ret + even) % 1_000_000_007;
            } else {
                even += 1;
                ret = (ret + odd) % 1_000_000_007;
            }
        }

        ret
    }
}
```
