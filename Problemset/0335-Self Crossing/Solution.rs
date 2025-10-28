impl Solution {
    pub fn is_self_crossing(mut distance: Vec<i32>) -> bool {
        let mut i = 2;

        while i < distance.len() {
            if distance[i] <= distance[i - 2] {
                let mut tmp = distance[i - 2];
                if i > 3 {
                    tmp -= distance[i - 4];
                }
                if i > 2 && distance[i] >= tmp {
                    distance[i - 1] -= distance[i - 3];
                }
                break;
            }

            i += 1;
        }

        while i < distance.len() - 1 {
            i += 1;

            if distance[i] >= distance[i - 2] {
                return true;
            }
        }

        false
    }
}
