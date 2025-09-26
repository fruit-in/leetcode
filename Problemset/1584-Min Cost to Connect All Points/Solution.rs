use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        let mut used = vec![false; points.len()];
        let mut heap = BinaryHeap::from([(Reverse(0), 0)]);
        let mut ret = 0;

        for _ in 0..points.len() {
            while let Some((Reverse(dist), i)) = heap.pop() {
                if !used[i] {
                    let (xi, yi) = (points[i][0], points[i][1]);
                    used[i] = true;
                    ret += dist;
                    for j in 0..points.len() {
                        if !used[j] {
                            let (xj, yj) = (points[j][0], points[j][1]);
                            let dist = (xi - xj).abs() + (yi - yj).abs();
                            heap.push((Reverse(dist), j));
                        }
                    }
                    break;
                }
            }
        }

        ret
    }
}
