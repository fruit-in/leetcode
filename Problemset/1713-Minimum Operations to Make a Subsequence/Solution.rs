use std::collections::HashMap;

impl Solution {
    pub fn min_operations(target: Vec<i32>, arr: Vec<i32>) -> i32 {
        let target = target
            .iter()
            .enumerate()
            .map(|(i, x)| (x, i))
            .collect::<HashMap<_, _>>();
        let arr = arr
            .iter()
            .filter(|x| target.contains_key(x))
            .map(|x| target[&x])
            .collect::<Vec<_>>();
        let mut lis = vec![];

        for i in &arr {
            if let Err(j) = lis.binary_search(&i) {
                if j == lis.len() {
                    lis.push(i);
                } else {
                    lis[j] = i;
                }
            }
        }

        (target.len() - lis.len()) as i32
    }
}
