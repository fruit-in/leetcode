# 1481. Least Number of Unique Integers after K Removals
Given an array of integers `arr` and an integer `k`. Find the *least number of unique integers* after removing **exactly** `k` elements.

#### Example 1:
<pre>
<strong>Input:</strong> arr = [5,5,4], k = 1
<strong>Output:</strong> 1
<strong>Explanation:</strong> Remove the single 4, only 5 is left.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> arr = [4,3,1,1,3,3,2], k = 3
<strong>Output:</strong> 2
<strong>Explanation:</strong> Remove 4, 2 and either one of the two 1s or three 3s. 1 and 3 will be left.
</pre>

#### Constraints:
* `1 <= arr.length <= 10^5`
* `1 <= arr[i] <= 10^9`
* `0 <= k <= arr.length`

## Solutions (Ruby)

### 1. Sort
```Ruby
# @param {Integer[]} arr
# @param {Integer} k
# @return {Integer}
def find_least_num_of_unique_ints(arr, k)
  counter = {}
  counter.default = 0
  arr.each { |x| counter[x] += 1 }
  counter = counter.values.sort

  (0..counter.size).each do |i|
    if k == 0
      return counter.size - i
    elsif k < 0
      return counter.size - i + 1
    end

    k -= counter[i]
  end
end
```

## Solutions (Rust)

### 1. Sort
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn find_least_num_of_unique_ints(arr: Vec<i32>, mut k: i32) -> i32 {
        let mut counter = HashMap::new();
        for x in arr {
            *counter.entry(x).or_insert(0) += 1;
        }

        let mut counter = counter.values().collect::<Vec<_>>();
        counter.sort_unstable();

        for i in 0..=counter.len() {
            if k == 0 {
                return (counter.len() - i) as i32;
            } else if k < 0 {
                return (counter.len() - i) as i32 + 1;
            }
            k -= counter[i];
        }

        0
    }
}
```
