use std::collections::HashMap;

impl Solution {
    pub fn number_of_boomerangs(points: Vec<Vec<i32>>) -> i32 {
        let mut len_cnt = HashMap::new();
        let mut ret = 0;

        for i in 0..points.len() {
            for j in 0..points.len() {
                let dx = points[i][0] - points[j][0];
                let dy = points[i][1] - points[j][1];
                *len_cnt.entry(dx * dx + dy * dy).or_insert(0) += 1;
            }

            ret += len_cnt.values().map(|v| v * (v - 1)).sum::<i32>();
            len_cnt.clear();
        }

        ret
    }
}
