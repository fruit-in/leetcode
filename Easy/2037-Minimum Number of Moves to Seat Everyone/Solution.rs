impl Solution {
    pub fn min_moves_to_seat(seats: Vec<i32>, students: Vec<i32>) -> i32 {
        let mut seats = seats;
        let mut students = students;
        seats.sort_unstable();
        students.sort_unstable();

        seats
            .into_iter()
            .zip(students.into_iter())
            .map(|(x, y)| (x - y).abs())
            .sum()
    }
}
