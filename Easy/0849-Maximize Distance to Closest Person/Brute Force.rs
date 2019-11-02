impl Solution {
    pub fn max_dist_to_closest(seats: Vec<i32>) -> i32 {
        let mut max_distance = 0;

        for i in 0..seats.len() {
            if seats[i] == 0 {
                let mut left_distance = 0;
                for j in (0..=i).rev() {
                    if seats[j] == 1 {
                        break;
                    }
                    left_distance += 1;
                    if j == 0 {
                        left_distance += 20000;
                    }
                }

                let mut right_distance = 0;
                for j in i..seats.len() {
                    if seats[j] == 1 {
                        break;
                    }
                    right_distance += 1;
                    if j == seats.len() - 1{
                        right_distance += 20000;
                    }
                }

                max_distance = max_distance.max(left_distance.min(right_distance));
            }
        }

        max_distance
    }
}
