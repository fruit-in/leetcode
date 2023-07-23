impl Solution {
    pub fn max_dist_to_closest(seats: Vec<i32>) -> i32 {
        let mut max_empty = 0;

        let mut i = -1;
        for j in 0..(seats.len() + 1) {
            if j == seats.len() {
                max_empty = max_empty.max(2 * (j as i32 - i - 1));
            } else if seats[j] == 1 {
                if i == -1 {
                    max_empty = max_empty.max(2 * (j as i32 - i - 1));
                } else {
                    max_empty = max_empty.max(j as i32 - i - 1);
                }
                i = j as i32;
            }
        }

        (max_empty + 1) / 2
    }
}
