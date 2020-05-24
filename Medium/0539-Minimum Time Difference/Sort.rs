impl Solution {
    pub fn find_min_difference(time_points: Vec<String>) -> i32 {
        let mut minutes = time_points
            .iter()
            .map(|time| time.split(':').map(|n| n.parse::<i32>().unwrap()))
            .map(|mut h_m| h_m.next().unwrap() * 60 + h_m.next().unwrap())
            .collect::<Vec<i32>>();

        minutes.sort_unstable();

        let mut ret = 1440 + minutes[0] - minutes.last().unwrap();

        for i in 0..(minutes.len() - 1) {
            ret = ret.min(minutes[i + 1] - minutes[i]);
        }

        ret
    }
}
