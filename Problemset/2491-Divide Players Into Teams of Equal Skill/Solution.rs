impl Solution {
    pub fn divide_players(skill: Vec<i32>) -> i64 {
        let n = skill.len();
        let mut skill = skill;
        let mut ret = 0;

        skill.sort_unstable();

        for i in 0..n / 2 {
            if skill[i] + skill[n - 1 - i] != skill[0] + skill[n - 1] {
                return -1;
            }

            ret += skill[i] as i64 * skill[n - 1 - i] as i64;
        }

        ret
    }
}
