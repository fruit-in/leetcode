use std::collections::BinaryHeap;

impl Solution {
    pub fn repair_cars(ranks: Vec<i32>, cars: i32) -> i64 {
        let mut heap = ranks
            .iter()
            .map(|&r| (-r as i64, r as i64, 0))
            .collect::<BinaryHeap<_>>();

        for _ in 0..cars {
            let mut time_rank_cars = heap.peek_mut().unwrap();
            time_rank_cars.2 += 1;
            time_rank_cars.0 = -time_rank_cars.1 * (time_rank_cars.2 + 1) * (time_rank_cars.2 + 1);
        }

        heap.iter().map(|&(t, r, n)| r * n * n).max().unwrap()
    }
}
