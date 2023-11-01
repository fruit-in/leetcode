# 2007. Find Original Array From Doubled Array
An integer array `original` is transformed into a **doubled** array `changed` by appending **twice the value** of every element in `original`, and then randomly **shuffling** the resulting array.

Given an array `changed`, return `original` *if* `changed` *is a **doubled** array. If* `changed` *is not a **doubled** array, return an empty array. The elements in* `original` *may be returned in **any** order*.

#### Example 1:
<pre>
<strong>Input:</strong> changed = [1,3,4,2,6,8]
<strong>Output:</strong> [1,3,4]
<strong>Explanation:</strong> One possible original array could be [1,3,4]:
- Twice the value of 1 is 1 * 2 = 2.
- Twice the value of 3 is 3 * 2 = 6.
- Twice the value of 4 is 4 * 2 = 8.
Other original arrays could be [4,3,1] or [3,1,4].
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> changed = [6,3,0,1]
<strong>Output:</strong> []
<strong>Explanation:</strong> changed is not a doubled array.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> changed = [1]
<strong>Output:</strong> []
<strong>Explanation:</strong> changed is not a doubled array.
</pre>

#### Constraints:
* <code>1 <= changed.length <= 10<sup>5</sup></code>
* <code>0 <= changed[i] <= 10<sup>5</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn find_original_array(changed: Vec<i32>) -> Vec<i32> {
        let mut changed = changed;
        let mut count = HashMap::new();
        let mut original = vec![];

        changed.sort_unstable();

        for num in &changed {
            if num % 2 == 0 && *count.get(&(num / 2)).unwrap_or(&0) > 0 {
                *count.get_mut(&(num / 2)).unwrap() -= 1;
                original.push(num / 2);
            } else {
                *count.entry(num).or_insert(0) += 1;
            }
        }

        if original.len() * 2 != changed.len() {
            original.clear();
        }

        original
    }
}
```
