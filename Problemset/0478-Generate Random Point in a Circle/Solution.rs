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
