# 2582. Pass the Pillow
There are `n` people standing in a line labeled from `1` to `n`. The first person in the line is holding a pillow initially. Every second, the person holding the pillow passes it to the next person standing in the line. Once the pillow reaches the end of the line, the direction changes, and people continue passing the pillow in the opposite direction.

* For example, once the pillow reaches the <code>n<sup>th</sup></code> person they pass it to the <code>n - 1<sup>th</sup></code> person, then to the <code>n - 2<sup>th</sup></code> person and so on.

Given the two positive integers `n` and `time`, return *the index of the person holding the pillow after* `time` *seconds*.

#### Example 1:
<pre>
<strong>Input:</strong> n = 4, time = 5
<strong>Output:</strong> 2
<strong>Explanation:</strong> People pass the pillow in the following way: 1 -> 2 -> 3 -> 4 -> 3 -> 2.
Afer five seconds, the pillow is given to the 2nd person.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 3, time = 2
<strong>Output:</strong> 3
<strong>Explanation:</strong> People pass the pillow in the following way: 1 -> 2 -> 3.
Afer two seconds, the pillow is given to the 3rd person.
</pre>

#### Constraints:
* `2 <= n <= 1000`
* `1 <= time <= 1000`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn pass_the_pillow(n: i32, time: i32) -> i32 {
        let mut dir = 1;
        let mut ret = 1;

        for _ in 0..time {
            ret += dir;

            if ret == 1 || ret == n {
                dir = -dir;
            }
        }

        ret
    }
}
```
