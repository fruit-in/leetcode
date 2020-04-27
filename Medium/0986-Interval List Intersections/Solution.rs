impl Solution {
    pub fn interval_intersection(a: Vec<Vec<i32>>, b: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut i = 0;
        let mut j = 0;
        let mut ret = Vec::new();

        while i < a.len() && j < b.len() {
            let max_l = a[i][0].max(b[j][0]);
            let min_r = a[i][1].min(b[j][1]);

            if min_r >= max_l {
                ret.push(vec![max_l, min_r]);
            }

            if min_r == a[i][1] {
                i += 1;
            } else {
                j += 1;
            }
        }

        ret
    }
}
