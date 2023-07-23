impl Solution {
    pub fn prison_after_n_days(mut cells: Vec<i32>, n: i32) -> Vec<i32> {
        Self::prison_after_one_day(&mut cells);

        let origin = cells.clone();
        let mut period = n;

        for i in 1..n {
            Self::prison_after_one_day(&mut cells);

            if cells[1..7] == origin[1..7] {
                period = i;
                break;
            }
        }

        for _ in 0..((n - 1 - period) % period) {
            Self::prison_after_one_day(&mut cells);
        }

        cells
    }

    pub fn prison_after_one_day(cells: &mut Vec<i32>) {
        let next_day = (1..7)
            .map(|i| 1 - (cells[i - 1] ^ cells[i + 1]))
            .collect::<Vec<_>>();

        cells[0] = 0;
        cells[7] = 0;
        for i in 1..7 {
            cells[i] = next_day[i - 1];
        }
    }
}
