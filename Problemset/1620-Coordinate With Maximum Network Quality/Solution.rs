impl Solution {
    pub fn best_coordinate(towers: Vec<Vec<i32>>, radius: i32) -> Vec<i32> {
        let min_x = towers.iter().map(|t| t[0]).min().unwrap();
        let min_y = towers.iter().map(|t| t[1]).min().unwrap();
        let max_x = towers.iter().map(|t| t[0]).max().unwrap();
        let max_y = towers.iter().map(|t| t[1]).max().unwrap();
        let mut max_q = 0;
        let mut ret = vec![0, 0];

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                let mut q = 0;

                for t in &towers {
                    let d = (((t[0] - x).pow(2) + (t[1] - y).pow(2)) as f64).sqrt();

                    if d <= radius as f64 {
                        q += (t[2] as f64 / (1.0 + d)) as i32;
                    }
                }

                if q > max_q {
                    max_q = q;
                    ret = vec![x, y];
                }
            }
        }

        ret
    }
}
