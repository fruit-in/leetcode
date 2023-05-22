impl Solution {
    pub fn min_number_of_hours(
        initial_energy: i32,
        initial_experience: i32,
        energy: Vec<i32>,
        experience: Vec<i32>,
    ) -> i32 {
        let mut initial_energy = initial_energy;
        let mut initial_experience = initial_experience;
        let mut ret = 0;

        for i in 0..energy.len() {
            if initial_energy <= energy[i] {
                ret += energy[i] + 1 - initial_energy;
                initial_energy = energy[i] + 1;
            }
            if initial_experience <= experience[i] {
                ret += experience[i] + 1 - initial_experience;
                initial_experience = experience[i] + 1;
            }

            initial_energy -= energy[i];
            initial_experience += experience[i];
        }

        ret
    }
}
