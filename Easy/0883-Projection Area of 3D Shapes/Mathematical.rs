impl Solution {
    pub fn projection_area(grid: Vec<Vec<i32>>) -> i32 {
        let mut top = 0;
        let mut front = 0;
        let mut side = 0;

        for x in 0..grid.len() {
            let mut front_max = 0;
            let mut side_max = 0;

            for y in 0..grid[0].len() {
                if grid[x][y] > 0 {
                    top += 1;
                }
                front_max = front_max.max(grid[x][y]);
                side_max = side_max.max(grid[y][x]);
            }

            front += front_max;
            side += side_max;
        }

        top + front + side
    }
}
