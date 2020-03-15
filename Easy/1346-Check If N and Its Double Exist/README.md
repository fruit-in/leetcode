# 1346. Check If N and Its Double Exist
Given an array ```arr``` of integers, check if there exists two integers ```N``` and ```M``` such that ```N``` is the double of ```M``` ( i.e. ```N = 2 * M```).

More formally check if there exists two indices ```i``` and ```j``` such that :
* ```i != j```
* ```0 <= i, j < arr.length```
* ```arr[i] == 2 * arr[j]```

#### Example 1:
<pre>
<strong>Input:</strong> arr = [10,2,5,3]
<strong>Output:</strong> true
<strong>Explanation:</strong> N = 10 is the double of M = 5,that is, 10 = 2 * 5.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> arr = [7,1,14,11]
<strong>Output:</strong> true
<strong>Explanation:</strong> N = 14 is the double of M = 7,that is, 14 = 2 * 7.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> arr = [3,1,7,11]
<strong>Output:</strong> false
<strong>Explanation:</strong> In this case does not exist N and M, such that N = 2 * M.
</pre>

#### Constraints:
* ```2 <= arr.length <= 500```
* ```-10^3 <= arr[i] <= 10^3```

## Solutions (Rust)

### 1. Set
```Rust
use std::collections::HashSet;

impl Solution {
    pub fn check_if_exist(arr: Vec<i32>) -> bool {
        let mut set = HashSet::new();

        for n in arr {
            if set.contains(&(2 * n)) || (n % 2 == 0 && set.contains(&(n / 2))) {
                return true;
            }
            set.insert(n);
        }

        false
    }
}
```
