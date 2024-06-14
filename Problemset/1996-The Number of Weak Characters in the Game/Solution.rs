impl Solution {
    pub fn number_of_weak_characters(mut properties: Vec<Vec<i32>>) -> i32 {
        let mut max_defense = 0;
        let mut ret = 0;

        properties.sort_unstable_by_key(|p| (-p[0], p[1]));

        for p in &properties {
            ret += (max_defense > p[1]) as i32;
            max_defense = max_defense.max(p[1]);
        }

        ret
    }
}
