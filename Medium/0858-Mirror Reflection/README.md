# 858. Mirror Reflection
There is a special square room with mirrors on each of the four walls.  Except for the southwest corner, there are receptors on each of the remaining corners, numbered `0`, `1`, and `2`.

The square room has walls of length `p`, and a laser ray from the southwest corner first meets the east wall at a distance `q` from the `0`th receptor.

Return the number of the receptor that the ray meets first.  (It is guaranteed that the ray will meet a receptor eventually.)

#### Example 1:
<pre>
<strong>Input:</strong> p = 2, q = 1
<strong>Output:</strong> 2
<strong>Explanation:</strong> The ray meets receptor 2 the first time it gets reflected back to the left wall.
<img src="https://s3-lc-upload.s3.amazonaws.com/uploads/2018/06/18/reflection.png">
</pre>

#### Note:
1. `1 <= p <= 1000`
2. `0 <= q <= p`

## Solutions (Rust)

### 1. Mathematical
```Rust
impl Solution {
    pub fn mirror_reflection(p: i32, q: i32) -> i32 {
        let mut a = p;
        let mut b = q;

        while b != 0 {
            let tmp = b;
            b = a % b;
            a = tmp;
        }

        match (p / a % 2, q / a % 2) {
            (0, _) => 2,
            (1, 0) => 0,
            (_, _) => 1,
        }
    }
}
```
