use std::collections::HashMap;

impl Solution {
    pub fn min_number_of_frogs(croak_of_frogs: String) -> i32 {
        let mut counter = vec![('c', 0), ('r', 0), ('o', 0), ('a', 0)]
            .into_iter()
            .collect::<HashMap<_, _>>();
        let mut prev = vec![('r', 'c'), ('o', 'r'), ('a', 'o')]
            .into_iter()
            .collect::<HashMap<_, _>>();
        let mut frogs = 0;
        let mut ret = 0;

        for c in croak_of_frogs.chars() {
            match c {
                'c' => {
                    *counter.get_mut(&'c').unwrap() += 1;
                    frogs += 1;
                    ret = ret.max(frogs);
                }
                'k' => {
                    if counter[&'a'] == 0 {
                        return -1;
                    }
                    *counter.get_mut(&'a').unwrap() -= 1;
                    frogs -= 1;
                }
                _ => {
                    if counter[&prev[&c]] == 0 {
                        return -1;
                    }
                    *counter.get_mut(&prev[&c]).unwrap() -= 1;
                    *counter.get_mut(&c).unwrap() += 1;
                }
            }
        }

        if counter.values().all(|&v| v == 0) {
            ret
        } else {
            -1
        }
    }
}
