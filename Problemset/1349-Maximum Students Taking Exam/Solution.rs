use std::collections::HashMap;

impl Solution {
    pub fn max_students(seats: Vec<Vec<char>>) -> i32 {
        let (m, n) = (seats.len(), seats[0].len());
        let mut prev = HashMap::from([(0, 0)]);

        for i in 0..m {
            let mut seats_mask = 0;
            let mut tmp = HashMap::new();

            for j in 0..n {
                if seats[i][j] == '.' {
                    seats_mask |= 1 << j;
                }
            }

            for students_mask in 0_i32..1 << n {
                if students_mask | seats_mask != seats_mask
                    || (0..n).any(|i| (students_mask >> i) & 3 == 3)
                {
                    continue;
                }

                for (&prev_students_mask, &students) in prev.iter() {
                    if (0..n).all(|i| ((students_mask | prev_students_mask) >> i) & 3 != 3)
                        && *tmp.get(&students_mask).unwrap_or(&0)
                            <= students_mask.count_ones() + students
                    {
                        tmp.insert(students_mask, students_mask.count_ones() + students);
                    }
                }
            }

            prev = tmp;
        }

        *prev.values().max().unwrap() as i32
    }
}
