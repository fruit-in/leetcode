# 1122. Relative Sort Array
Given two arrays ```arr1``` and ```arr2```, the elements of ```arr2``` are distinct, and all elements in ```arr2``` are also in ```arr1```.

Sort the elements of ```arr1``` such that the relative ordering of items in ```arr1``` are the same as in ```arr2```.  Elements that don't appear in ```arr2``` should be placed at the end of ```arr1``` in **ascending** order.

#### Example 1:
<pre>
<strong>Input:</strong> arr1 = [2,3,1,3,2,4,6,7,9,2,19], arr2 = [2,1,4,3,9,6]
<strong>Output:</strong> [2,2,2,1,4,3,3,9,6,7,19]
</pre>

#### Constraints:
* ```arr1.length, arr2.length <= 1000```
* ```0 <= arr1[i], arr2[i] <= 1000```
* Each ```arr2[i]``` is distinct.
* Each ```arr2[i]``` is in ```arr1```.

## Solutions (Rust)

### 1. Sort by Different Keys
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let map: HashMap<i32, usize> = arr2.iter().enumerate()
            .map(|(i, v)| (*v, i))
            .collect();
 
        let mut v1: Vec<i32> = Vec::new();
        let mut v2: Vec<i32> = Vec::new();
        for n in arr1 {
            match map.contains_key(&n) {
                true => v2.push(n),
                false => v1.push(n),
            };
        }
 
        v1.sort_unstable();
        v2.sort_unstable_by_key(|n| *map.get(&n).unwrap());
        v2.append(&mut v1);
        v2
    }
}
```

### 2. Counting Sort
```Rust
impl Solution {
    pub fn relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let mut arr3: Vec<i32> = (0..=1000)
            .filter(|n| arr1.contains(n) && !arr2.contains(n))
            .collect();
 
        let mut arr2: Vec<i32> = arr2;
        arr2.append(&mut arr3);
 
        let counter: Vec<usize> = arr2.iter()
            .map(|n| arr1.iter().filter(|&m| m == n).count())
            .collect();
 
        let mut ret: Vec<i32> = Vec::new();
        for i in 0..arr2.len() {
            for j in 0..counter[i] {
                ret.push(arr2[i]);
            }
        }
        ret
    }
}
```
