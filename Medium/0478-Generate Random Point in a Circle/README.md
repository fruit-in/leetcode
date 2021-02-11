# 478. Generate Random Point in a Circle
Given the radius and x-y positions of the center of a circle, write a function `randPoint` which generates a uniform random point in the circle.

Note:
1. input and output values are in [floating-point](https://www.webopedia.com/definitions/floating-point-number/).
2. radius and x-y position of the center of the circle is passed into the class constructor.
3. a point on the circumference of the circle is considered to be in the circle.
4. `randPoint` returns a size 2 array containing x-position and y-position of the random point, in that order.

#### Example 1:
<pre>
<strong>Input:</strong>
["Solution","randPoint","randPoint","randPoint"]
[[1,0,0],[],[],[]]
<strong>Output:</strong> [null,[-0.72939,-0.65505],[-0.78502,-0.28626],[-0.83119,-0.19803]]
</pre>

#### Example 2:
<pre>
<strong>Input:</strong>
["Solution","randPoint","randPoint","randPoint"]
[[10,5,-7.5],[],[],[]]
<strong>Output:</strong> [null,[11.52438,-8.33273],[2.46992,-16.21705],[11.13430,-12.42337]]
</pre>

#### Explanation of Input Syntax:
The input is two lists: the subroutines called and their arguments. `Solution`'s constructor has three arguments, the radius, x-position of the center, and y-position of the center of the circle. `randPoint` has no arguments. Arguments are always wrapped with a list, even if there aren't any.

## Solutions (Rust)

### 1. Solution
```Rust
use rand::{thread_rng, Rng};

struct Solution {
    radius: f64,
    x_center: f64,
    y_center: f64,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(radius: f64, x_center: f64, y_center: f64) -> Self {
        Self {
            radius,
            x_center,
            y_center,
        }
    }

    fn rand_point(&self) -> Vec<f64> {
        let mut rng = thread_rng();
        let delta_x = rng.gen_range(-self.radius, self.radius);
        let delta_y = rng.gen_range(-self.radius, self.radius);

        if delta_x.powi(2) + delta_y.powi(2) <= self.radius.powi(2) {
            vec![self.x_center + delta_x, self.y_center + delta_y]
        } else {
            self.rand_point()
        }
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(radius, x_center, y_center);
 * let ret_1: Vec<f64> = obj.rand_point();
 */
```
