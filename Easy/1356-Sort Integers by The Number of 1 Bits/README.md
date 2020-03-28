# 1356. Sort Integers by The Number of 1 Bits
Given an integer array ```arr```. You have to sort the integers in the array in ascending order by the number of **1's** in their binary representation and in case of two or more integers have the same number of **1's** you have to sort them in ascending order.

Return *the sorted array*.

#### Example 1:
<pre>
<strong>Input:</strong> arr = [0,1,2,3,4,5,6,7,8]
<strong>Output:</strong> [0,1,2,4,8,3,5,6,7]
<strong>Explanation:</strong> [0] is the only integer with 0 bits.
[1,2,4,8] all have 1 bit.
[3,5,6] have 2 bits.
[7] has 3 bits.
The sorted array by bits is [0,1,2,4,8,3,5,6,7]
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> arr = [1024,512,256,128,64,32,16,8,4,2,1]
<strong>Output:</strong> [1,2,4,8,16,32,64,128,256,512,1024]
<strong>Explanation:</strong> All integers have 1 bit in the binary representation, you should just sort them in ascending order.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> arr = [10000,10000]
<strong>Output:</strong> [10000,10000]
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> arr = [2,3,5,7,11,13,17,19]
<strong>Output:</strong> [2,3,5,17,7,11,13,19]
</pre>

#### Example 5:
<pre>
<strong>Input:</strong> arr = [10,100,1000,10000]
<strong>Output:</strong> [10,100,10000,1000]
</pre>

#### Constraints:
* ```1 <= arr.length <= 500```
* ```0 <= arr[i] <= 10^4```

## Solutions (Rust)

### 1. Sort
```Rust
impl Solution {
    pub fn sort_by_bits(arr: Vec<i32>) -> Vec<i32> {
        let mut arr = arr;
        arr.sort_unstable();
        arr.sort_by_key(|x| x.count_ones());
        arr
    }
}
```
