use std::collections::HashSet;

impl Solution {
    pub fn min_area_rect(mut points: Vec<Vec<i32>>) -> i32 {
        let mut set = HashSet::new();
        let mut min_area = None;
        points.sort_unstable();

        for i in 0..points.len() {
            let (x0, y0) = (points[i][0], points[i][1]);

            for j in 0..i {
                let (x1, y1) = (points[j][0], points[j][1]);

                if set.contains(&(x0, y1)) && set.contains(&(x1, y0)) {
                    let area = (x0 - x1) * (y0 - y1).abs();
                    min_area = Some(min_area.map_or(area, |a: i32| a.min(area)));
                }
            }

            set.insert((x0, y0));
        }

        min_area.unwrap_or(0)
    }
}
