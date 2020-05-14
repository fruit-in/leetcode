use std::collections::HashMap;

impl Solution {
    pub fn get_kth(lo: i32, hi: i32, k: i32) -> i32 {
        let mut arr = (lo..=hi).collect::<Vec<i32>>();
        let mut stack = arr.clone();
        let mut power = HashMap::new();
        power.insert(1, 0);

        while let Some(x) = stack.pop() {
            if !power.contains_key(&x) {
                let y = match x % 2 {
                    0 => x / 2,
                    _ => 3 * x + 1,
                };
                match power.get(&y) {
                    Some(z) => { power.insert(x, z + 1); },
                    None => {
                        stack.push(x);
                        stack.push(y);
                    },
                }
            }
        }

        arr.sort_by_key(|x| *power.get(&x).unwrap());

        arr[k as usize - 1]
    }
}
