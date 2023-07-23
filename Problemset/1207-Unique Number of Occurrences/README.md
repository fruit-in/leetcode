# 1207. Unique Number of Occurrences
Given an array of integers ```arr```, write a function that returns ```true``` if and only if the number of occurrences of each value in the array is unique.

#### Example 1:
<pre>
<strong>Input:</strong> arr = [1,2,2,1,1,3]
<strong>Output:</strong> true
<strong>Explanation:</strong> The value 1 has 3 occurrences, 2 has 2 and 3 has 1. No two values have the same number of occurrences.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> arr = [1,2]
<strong>Output:</strong> false
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> arr = [-3,0,1,-3,1,1,1,-3,10,0]
<strong>Output:</strong> true
</pre>

#### Constraints:
* ```1 <= arr.length <= 1000```
* ```-1000 <= arr[i] <= 1000```

## Solutions (Rust)

### 1. HashMap & Set
```Rust
use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut map = HashMap::new();
        for n in arr {
            *map.entry(n).or_insert(0) += 1;
        }

        let set: HashSet<_> = map.values().collect();

        set.len() == map.len()
    }
}
```

### 2. Sort
```Rust
impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut arr = arr;
        arr.sort_unstable();

        let mut times = [false; 1001];
        let mut i = 0;
        let mut j = 1;
        while i < arr.len() {
            while j < arr.len() && arr[i] == arr[j] {
                j += 1;
            }
            if times[j - i] {
                return false;
            }
            times[j - i] = true;
            i = j;
        }

        true
    }
}
```
