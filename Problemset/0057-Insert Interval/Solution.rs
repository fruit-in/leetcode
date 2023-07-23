impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, mut new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut flag = true;
        let mut ret = vec![];

        for interval in intervals {
            if interval[0] > new_interval[1] {
                if flag {
                    ret.push(new_interval.clone());
                    flag = false;
                }
                ret.push(interval);
            } else if interval[1] < new_interval[0] {
                ret.push(interval);
            } else {
                new_interval[0] = new_interval[0].min(interval[0]);
                new_interval[1] = new_interval[1].max(interval[1]);
            }
        }
        if flag {
            ret.push(new_interval);
        }

        ret
    }
}
