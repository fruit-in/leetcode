# 2167. Minimum Time to Remove All Cars Containing Illegal Goods
You are given a **0-indexed** binary string `s` which represents a sequence of train cars. `s[i] = '0'` denotes that the <code>i<sup>th</sup></code> car does **not** contain illegal goods and `s[i] = '1'` denotes that the <code>i<sup>th</sup></code> car does contain illegal goods.

As the train conductor, you would like to get rid of all the cars containing illegal goods. You can do any of the following three operations **any** number of times:

1. Remove a train car from the **left** end (i.e., remove `s[0]`) which takes 1 unit of time.
2. Remove a train car from the **right** end (i.e., remove `s[s.length - 1]`) which takes 1 unit of time.
3. Remove a train car from **anywhere** in the sequence which takes 2 units of time.

Return *the **minimum** time to remove all the cars containing illegal goods*.

Note that an empty sequence of cars is considered to have no cars containing illegal goods.

#### Example 1:
<pre>
<strong>Input:</strong> s = "1100101"
<strong>Output:</strong> 5
<strong>Explanation:</strong>
One way to remove all the cars containing illegal goods from the sequence is to
- remove a car from the left end 2 times. Time taken is 2 * 1 = 2.
- remove a car from the right end. Time taken is 1.
- remove the car containing illegal goods found in the middle. Time taken is 2.
This obtains a total time of 2 + 1 + 2 = 5.

An alternative way is to
- remove a car from the left end 2 times. Time taken is 2 * 1 = 2.
- remove a car from the right end 3 times. Time taken is 3 * 1 = 3.
This also obtains a total time of 2 + 3 = 5.

5 is the minimum time taken to remove all the cars containing illegal goods.
There are no other ways to remove them with less time.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "0010"
<strong>Output:</strong> 2
<strong>Explanation:</strong>
One way to remove all the cars containing illegal goods from the sequence is to
- remove a car from the left end 3 times. Time taken is 3 * 1 = 3.
This obtains a total time of 3.

Another way to remove all the cars containing illegal goods from the sequence is to
- remove the car containing illegal goods found in the middle. Time taken is 2.
This obtains a total time of 2.

Another way to remove all the cars containing illegal goods from the sequence is to
- remove a car from the right end 2 times. Time taken is 2 * 1 = 2.
This obtains a total time of 2.

2 is the minimum time taken to remove all the cars containing illegal goods.
There are no other ways to remove them with less time.
</pre>

#### Constraints:
* <code>1 <= s.length <= 2 * 10<sup>5</sup></code>
* `s[i]` is either `'0'` or `'1'`.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn minimum_time(s: String) -> i32 {
        let s = s.as_bytes();
        let mut count = 0;
        let mut x = 1;
        let mut indices_r = vec![];
        let mut ret = i32::MAX;

        for i in (0..s.len()).rev() {
            if s[i] == b'1' {
                indices_r.push(i);
            }
        }

        if indices_r.is_empty() {
            return 0;
        }

        for i in 0..s.len() {
            if s[i] == b'1' {
                indices_r.pop();
            }
            count += (s[i] - b'0') as i32;
            x = x.min(i as i32 - 2 * count + 2);
            ret = ret
                .min(2 * count - 1 + x + (s.len() - indices_r.last().unwrap_or(&s.len())) as i32);
        }

        ret
    }
}
```
