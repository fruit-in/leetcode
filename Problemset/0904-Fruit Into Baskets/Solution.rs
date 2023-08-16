use std::collections::HashMap;

impl Solution {
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        let mut count = HashMap::from([(fruits[0], 1)]);
        let mut r = 0;
        let mut ret = 0;

        for l in 0..fruits.len() {
            while count.len() <= 2 {
                ret = ret.max(r - l + 1);

                if r + 1 == fruits.len() {
                    break;
                }
                r += 1;
                count.entry(fruits[r]).and_modify(|c| *c += 1).or_insert(1);
            }

            if count[&fruits[l]] == 1 {
                count.remove(&fruits[l]);
            } else {
                *count.get_mut(&fruits[l]).unwrap() -= 1;
            }
        }

        ret as i32
    }
}
