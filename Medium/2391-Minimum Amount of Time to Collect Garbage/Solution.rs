impl Solution {
    pub fn garbage_collection(garbage: Vec<String>, travel: Vec<i32>) -> i32 {
        let mut time = [0; 3];
        let mut count = [0; 3];
        let mut ret = 0;

        for i in 0..garbage.len() {
            count = [0; 3];
            for g in garbage[i].chars() {
                match g {
                    'M' => count[0] += 1,
                    'P' => count[1] += 1,
                    _ => count[2] += 1,
                }
            }

            for j in 0..3 {
                if count[j] > 0 {
                    ret += time[j] + count[j];
                    time[j] = 0;
                }
            }

            time[0] += travel.get(i).unwrap_or(&0);
            time[1] += travel.get(i).unwrap_or(&0);
            time[2] += travel.get(i).unwrap_or(&0);
        }

        ret
    }
}
