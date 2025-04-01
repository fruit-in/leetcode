use std::collections::HashMap;

impl Solution {
    pub fn smallest_sufficient_team(req_skills: Vec<String>, people: Vec<Vec<String>>) -> Vec<i32> {
        let mut skill_indices = HashMap::new();
        let mut min_team = HashMap::from([(0, vec![])]);

        for i in 0..req_skills.len() {
            skill_indices.insert(&req_skills[i], i);
        }

        for i in 0..people.len() {
            let mut person_bin = 0;

            for skill in &people[i] {
                person_bin |= 1 << skill_indices[&skill];
            }

            for (k, mut v) in min_team.clone().into_iter() {
                match min_team.get_mut(&(k | person_bin)) {
                    Some(team) if team.len() <= v.len() + 1 => (),
                    Some(team) => {
                        v.push(i as i32);
                        *team = v;
                    }
                    None => {
                        v.push(i as i32);
                        min_team.insert(k | person_bin, v);
                    }
                }
            }
        }

        min_team[&((1 << req_skills.len()) - 1)].clone()
    }
}
