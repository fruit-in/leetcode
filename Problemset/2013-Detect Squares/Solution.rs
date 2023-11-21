use std::collections::HashMap;

struct DetectSquares {
    points: HashMap<(i32, i32), i32>,
    diagonals: HashMap<(i32, i32), Vec<(i32, i32)>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl DetectSquares {
    fn new() -> Self {
        Self {
            points: HashMap::new(),
            diagonals: HashMap::new(),
        }
    }

    fn add(&mut self, point: Vec<i32>) {
        let (x, y) = (point[0], point[1]);

        *self.points.entry((x, y)).or_insert(0) += 1;
        self.diagonals
            .entry((1, y - x))
            .or_insert(vec![])
            .push((x, y));
        self.diagonals
            .entry((-1, y + x))
            .or_insert(vec![])
            .push((x, y));
    }

    fn count(&self, point: Vec<i32>) -> i32 {
        let (x0, y0) = (point[0], point[1]);
        let mut ret = 0;

        for &(x1, y1) in self.diagonals.get(&(1, y0 - x0)).unwrap_or(&vec![]) {
            if x1 == x0 {
                continue;
            }

            let count0 = *self.points.get(&(x0, y1)).unwrap_or(&0);
            let count1 = *self.points.get(&(x1, y0)).unwrap_or(&0);
            ret += count0 * count1;
        }
        for &(x1, y1) in self.diagonals.get(&(-1, y0 + x0)).unwrap_or(&vec![]) {
            if x1 == x0 {
                continue;
            }

            let count0 = *self.points.get(&(x0, y1)).unwrap_or(&0);
            let count1 = *self.points.get(&(x1, y0)).unwrap_or(&0);
            ret += count0 * count1;
        }

        ret
    }
}

/**
 * Your DetectSquares object will be instantiated and called as such:
 * let obj = DetectSquares::new();
 * obj.add(point);
 * let ret_2: i32 = obj.count(point);
 */
