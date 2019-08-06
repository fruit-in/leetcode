# 852. Peak Index in a Mountain Array
Let's call an array <code>A</code> a *mountain* if the following properties hold:
* <code>A.length >= 3</code>
* There exists some <code>0 < i < A.length - 1</code> such that <code>A[0] < A[1] < ... A[i-1] < A[i] > A[i+1] > ... > A[A.length - 1]</code>

Given an array that is definitely a mountain, return any <code>i</code> such that <code>A[0] < A[1] < ... A[i-1] < A[i] > A[i+1] > ... > A[A.length - 1]</code>.

#### Example 1:
<pre>
<strong>Input:</strong> [0,1,0]
<strong>Output:</strong> 1
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> [0,2,1,0]
<strong>Output:</strong> 1
</pre>

#### Note:
1. <code>3 <= A.length <= 10000</code>
2. <code>0 <= A[i] <= 10<sup>6</sup></code>
3. A is a mountain, as defined above.

## Solutions (Rust)

### 1. Binary Search
```Rust
impl Solution {
    pub fn peak_index_in_mountain_array(a: Vec<i32>) -> i32 {
        let mut head: usize = 0;
        let mut tail = a.len() - 1;
        let mut mid: usize;
        loop {
            mid = (head + tail) / 2;
            if a[mid - 1] < a[mid] && a[mid] > a[mid + 1] {
                return mid as i32;
            } else if a[mid] < a[mid + 1] {
                head = mid;
            } else if a[mid] > a[mid + 1] {
                tail = mid;
            }
        }
    }
}
```
## Solutions (Python)

### 1. Linear Scan
```Python3
class Solution:
    def peakIndexInMountainArray(self, A: List[int]) -> int:
        for i in range(1, len(A) - 1):
            if A[i] > A[i + 1]:
                return i
```
