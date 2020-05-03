# 1385. Find the Distance Value Between Two Arrays
Given two integer arrays ```arr1``` and ```arr2```, and the integer ```d```, *return the distance value between the two arrays*.

The distance value is defined as the number of elements ```arr1[i]``` such that there is not any element ```arr2[j]``` where ```|arr1[i]-arr2[j]| <= d```.

#### Example 1:
<pre>
<strong>Input:</strong> arr1 = [4,5,8], arr2 = [10,9,1,8], d = 2
<strong>Output:</strong> 2
<strong>Explanation:</strong>
For arr1[0]=4 we have:
|4-10|=6 > d=2
|4-9|=5 > d=2
|4-1|=3 > d=2
|4-8|=4 > d=2
For arr1[1]=5 we have:
|5-10|=5 > d=2
|5-9|=4 > d=2
|5-1|=4 > d=2
|5-8|=3 > d=2
For arr1[2]=8 we have:
<strong>|8-10|=2 <= d=2</strong>
<strong>|8-9|=1 <= d=2</strong>
|8-1|=7 > d=2
<strong>|8-8|=0 <= d=2</strong>
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> arr1 = [1,4,2,3], arr2 = [-4,-3,6,10,20,30], d = 3
<strong>Output:</strong> 2
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> arr1 = [2,1,100,3], arr2 = [-5,-2,10,-3,7], d = 6
<strong>Output:</strong> 1
</pre>

#### Constraints:
* ```1 <= arr1.length, arr2.length <= 500```
* ```-10^3 <= arr1[i], arr2[j] <= 10^3```
* ```0 <= d <= 100```

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn find_the_distance_value(arr1: Vec<i32>, arr2: Vec<i32>, d: i32) -> i32 {
        arr1.iter()
            .filter(|&x| arr2.iter().all(|y| (x - y).abs() > d))
            .count() as i32
    }
}
```
