use std::collections::VecDeque;

impl Solution {
    pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        let mut students = students.into_iter().collect::<VecDeque<_>>();
        let mut count = [0, 0];
        let mut i = 0;

        for &prefer in &students {
            count[prefer as usize] += 1;
        }

        while let Some(prefer) = students.pop_front() {
            if prefer == sandwiches[i] {
                i += 1;
                count[prefer as usize] -= 1;
            } else if count[0] == 0 || count[1] == 0 {
                students.push_back(prefer);
                break;
            } else {
                students.push_back(prefer);
            }
        }

        students.len() as i32
    }
}
