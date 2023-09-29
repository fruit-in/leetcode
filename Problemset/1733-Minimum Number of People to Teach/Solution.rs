use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn minimum_teachings(n: i32, languages: Vec<Vec<i32>>, friendships: Vec<Vec<i32>>) -> i32 {
        let mut teach_users: HashMap<i32, HashSet<i32>> = HashMap::new();

        for friendship in &friendships {
            let u = friendship[0];
            let v = friendship[1];
            let languages_u = languages[u as usize - 1].iter().collect::<HashSet<_>>();
            let languages_v = languages[v as usize - 1].iter().collect::<HashSet<_>>();

            if languages_u.intersection(&languages_v).count() == 0 {
                for language in 1..=n {
                    if !languages_u.contains(&&language) {
                        teach_users
                            .entry(language)
                            .or_insert(HashSet::new())
                            .insert(u);
                    }
                    if !languages_v.contains(&&language) {
                        teach_users
                            .entry(language)
                            .or_insert(HashSet::new())
                            .insert(v);
                    }
                }
            }
        }

        teach_users.values().map(|u| u.len()).min().unwrap_or(0) as i32
    }
}
