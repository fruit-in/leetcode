use std::collections::BinaryHeap;
use std::collections::HashMap;

impl Solution {
    pub fn alert_names(key_name: Vec<String>, key_time: Vec<String>) -> Vec<String> {
        let mut used_time = HashMap::new();
        let mut ret = BinaryHeap::new();

        for i in 0..key_name.len() {
            let h = key_time[i].get(..2).unwrap().parse::<i32>().unwrap();
            let m = key_time[i].get(3..).unwrap().parse::<i32>().unwrap();

            used_time
                .entry(&key_name[i])
                .or_insert(BinaryHeap::new())
                .push(h * 60 + m);
        }

        for (name, time) in used_time.into_iter() {
            let time = time.into_sorted_vec();

            for i in 2..time.len() {
                if time[i] - time[i - 2] <= 60 {
                    ret.push(name.to_string());
                    break;
                }
            }
        }

        ret.into_sorted_vec()
    }
}
