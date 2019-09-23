# 1200. Minimum Absolute Difference
Given an array of **distinct** integers ```arr```, find all pairs of elements with the minimum absolute difference of any two elements.

Return a list of pairs in ascending order(with respect to pairs), each pair ```[a, b]``` follows
* ```a, b``` are from ```arr```
* ```a < b```
* ```b - a``` equals to the minimum absolute difference of any two elements in ```arr```

#### Example 1:
<pre>
<strong>Input:</strong> arr = [4,2,1,3]
<strong>Output:</strong> [[1,2],[2,3],[3,4]]
<strong>Explanation:</strong> The minimum absolute difference is 1.
List all pairs with difference equal to 1 in ascending order.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> arr = [1,3,6,10,15]
<strong>Output:</strong> [[1,3]]
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> arr = [3,8,-10,23,19,-4,-14,27]
<strong>Output:</strong> [[-14,-10],[19,23],[23,27]]
</pre>

#### Constraints:
* ```2 <= arr.length <= 10^5```
* ```-10^6 <= arr[i] <= 10^6```

## Solutions (Rust)

### 1. Sort
```Rust
impl Solution {
    pub fn minimum_abs_difference(arr: Vec<i32>) -> Vec<Vec<i32>> {
        let mut arr = arr;
        arr.sort_unstable();
        let mut min = std::i32::MAX;
        let mut ret = Vec::new();

        for i in 1..arr.len() {
            if arr[i] - arr[i - 1] < min {
                min = arr[i] - arr[i - 1];
                ret.clear();
                ret.push(vec![arr[i - 1], arr[i]]);
            } else if arr[i] - arr[i - 1] == min {
                ret.push(vec![arr[i - 1], arr[i]]);
            }
        }

        ret
    }
}
```
