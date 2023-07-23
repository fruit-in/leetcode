# 481. Magical String
A magical string **S** consists of only '1' and '2' and obeys the following rules:

The string **S** is magical because concatenating the number of contiguous occurrences of characters '1' and '2' generates the string **S** itself.

The first few elements of string **S** is the following: **S** = "1221121221221121122……"

If we group the consecutive '1's and '2's in **S**, it will be:

1 22 11 2 1 22 1 22 11 2 11 22 ......

and the occurrences of '1's or '2's in each group are:

1 2 2 1 1 2 1 2 2 1 2 2 ......

You can see that the occurrence sequence above is the **S** itself.

Given an integer N as input, return the number of '1's in the first N number in the magical string **S**.

**Note:** N will not exceed 100,000.

#### Example 1:
<pre>
<strong>Input:</strong> 6
<strong>Output:</strong> 3
<strong>Explanation:</strong> The first 6 elements of magical string S is "12211" and it contains three 1's, so return 3.
</pre>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn magical_string(n: i32) -> i32 {
        let mut magical = vec![];
        let mut elem = true;
        let mut ret = 0;

        for i in 0..(n as usize) {
            magical.push(elem);
            match magical[i] {
                true => ret += 1,
                false => magical.push(elem),
            }
            elem = !elem;
        }

        ret
    }
}
```
