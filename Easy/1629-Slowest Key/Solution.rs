impl Solution {
    pub fn slowest_key(release_times: Vec<i32>, keys_pressed: String) -> char {
        let keys_pressed = keys_pressed.chars().collect::<Vec<_>>();
        let mut max_duration = release_times[0];
        let mut ret = keys_pressed[0];

        for i in 1..keys_pressed.len() {
            let duration = release_times[i] - release_times[i - 1];

            if duration > max_duration || (duration == max_duration && keys_pressed[i] > ret) {
                max_duration = duration;
                ret = keys_pressed[i];
            }
        }

        ret
    }
}
