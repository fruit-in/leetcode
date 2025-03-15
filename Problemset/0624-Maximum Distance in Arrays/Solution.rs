impl Solution {
    pub fn max_distance(arrays: Vec<Vec<i32>>) -> i32 {
        let mut min_num = arrays[0][0];
        let mut max_num = *arrays[0].last().unwrap();
        let mut ret = 0;

        for i in 1..arrays.len() {
            ret = ret
                .max(*arrays[i].last().unwrap() - min_num)
                .max(max_num - arrays[i][0]);
            min_num = min_num.min(arrays[i][0]);
            max_num = max_num.max(*arrays[i].last().unwrap());
        }

        ret
    }
}
