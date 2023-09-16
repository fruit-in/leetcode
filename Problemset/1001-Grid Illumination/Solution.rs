use std::collections::HashMap;

impl Solution {
    pub fn grid_illumination(n: i32, lamps: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut on_lamps = HashMap::new();
        let mut illuminated_rows = HashMap::new();
        let mut illuminated_cols = HashMap::new();
        let mut illuminated_dias0 = HashMap::new();
        let mut illuminated_dias1 = HashMap::new();
        let mut ans = vec![0; queries.len()];

        for lamp in &lamps {
            let row = lamp[0];
            let col = lamp[1];
            let dia0 = row - col;
            let dia1 = row + col;

            if on_lamps.insert((row, col), (dia0, dia1)).is_none() {
                illuminated_rows
                    .entry(row)
                    .and_modify(|c| *c += 1)
                    .or_insert(1);
                illuminated_cols
                    .entry(col)
                    .and_modify(|c| *c += 1)
                    .or_insert(1);
                illuminated_dias0
                    .entry(dia0)
                    .and_modify(|c| *c += 1)
                    .or_insert(1);
                illuminated_dias1
                    .entry(dia1)
                    .and_modify(|c| *c += 1)
                    .or_insert(1);
            }
        }

        for i in 0..queries.len() {
            let row = queries[i][0];
            let col = queries[i][1];
            let dia0 = row - col;
            let dia1 = row + col;

            ans[i] = (*illuminated_rows.get(&row).unwrap_or(&0) > 0
                || *illuminated_cols.get(&col).unwrap_or(&0) > 0
                || *illuminated_dias0.get(&dia0).unwrap_or(&0) > 0
                || *illuminated_dias1.get(&dia1).unwrap_or(&0) > 0) as i32;

            for x in -1..2 {
                for y in -1..2 {
                    if let Some((dia0, dia1)) = on_lamps.remove(&(row + x, col + y)) {
                        *illuminated_rows.get_mut(&(row + x)).unwrap() -= 1;
                        *illuminated_cols.get_mut(&(col + y)).unwrap() -= 1;
                        *illuminated_dias0.get_mut(&dia0).unwrap() -= 1;
                        *illuminated_dias1.get_mut(&dia1).unwrap() -= 1;
                    }
                }
            }
        }

        ans
    }
}
