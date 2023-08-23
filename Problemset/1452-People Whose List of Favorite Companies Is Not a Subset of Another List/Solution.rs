use std::collections::HashSet;

impl Solution {
    pub fn people_indexes(favorite_companies: Vec<Vec<String>>) -> Vec<i32> {
        let favorite_companies = favorite_companies
            .iter()
            .map(|v| v.iter().collect::<HashSet<_>>())
            .collect::<Vec<_>>();
        let mut flag = true;
        let mut ret = vec![];

        for i in 0..favorite_companies.len() {
            flag = true;

            for j in (0..i).chain(i + 1..favorite_companies.len()) {
                if favorite_companies[i].is_subset(&favorite_companies[j]) {
                    flag = false;
                    break;
                }
            }

            if flag {
                ret.push(i as i32);
            }
        }

        ret
    }
}
