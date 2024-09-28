use std::collections::HashSet;

impl Solution {
    pub fn min_area_free_rect(points: Vec<Vec<i32>>) -> f64 {
        let points_set = points.iter().map(|p| (p[0], p[1])).collect::<HashSet<_>>();
        let mut ret = f64::NAN;

        for i in 0..points.len() {
            let (xi, yi) = (points[i][0], points[i][1]);

            for j in i + 1..points.len() {
                let (xj, yj) = (points[j][0], points[j][1]);
                let ij2 = (xi - xj).pow(2) as f64 + (yi - yj).pow(2) as f64;

                for k in j + 1..points.len() {
                    let (xk, yk) = (points[k][0], points[k][1]);
                    let ik2 = (xi - xk).pow(2) as f64 + (yi - yk).pow(2) as f64;
                    let jk2 = (xj - xk).pow(2) as f64 + (yj - yk).pow(2) as f64;

                    if ij2 + ik2 == jk2 && points_set.contains(&(xj + xk - xi, yj + yk - yi)) {
                        ret = ret.min(ij2.sqrt() * ik2.sqrt());
                    } else if ij2 + jk2 == ik2 && points_set.contains(&(xi + xk - xj, yi + yk - yj))
                    {
                        ret = ret.min(ij2.sqrt() * jk2.sqrt());
                    } else if ik2 + jk2 == ij2 && points_set.contains(&(xi + xj - xk, yi + yj - yk))
                    {
                        ret = ret.min(ik2.sqrt() * jk2.sqrt());
                    }
                }
            }
        }

        if ret.is_nan() {
            return 0.;
        }

        ret
    }
}
