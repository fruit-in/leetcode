# 1122. 数组的相对排序
给你两个数组，```arr1``` 和 ```arr2```，
* ```arr2``` 中的元素各不相同
* ```arr2``` 中的每个元素都出现在 ```arr1``` 中

对 ```arr1``` 中的元素进行排序，使 ```arr1``` 中项的相对顺序和 ```arr2``` 中的相对顺序相同。未在 ```arr2``` 中出现过的元素需要按照升序放在 ```arr1``` 的末尾。

#### 示例:
<pre>
<strong>输入:</strong> arr1 = [2,3,1,3,2,4,6,7,9,2,19], arr2 = [2,1,4,3,9,6]
<strong>输出:</strong> [2,2,2,1,4,3,3,9,6,7,19]
</pre>

#### 提示:
* ```arr1.length, arr2.length <= 1000```
* ```0 <= arr1[i], arr2[i] <= 1000```
* ```arr2``` 中的元素 ```arr2[i]``` 各不相同
* ```arr2``` 中的每个元素 ```arr2[i]``` 都出现在 ```arr1``` 中

## 题解 (Rust)

### 1. 根据键排序
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

### 2. 计数排序
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
