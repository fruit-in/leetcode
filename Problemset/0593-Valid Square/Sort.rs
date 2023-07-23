impl Solution {
    pub fn valid_square(p1: Vec<i32>, p2: Vec<i32>, p3: Vec<i32>, p4: Vec<i32>) -> bool {
        let points = [p1, p2, p3, p4];
        let mut dis_2 = Vec::new();

        for i in 1..4 {
            for j in 0..i {
                let x = points[i][0] - points[j][0];
                let y = points[i][1] - points[j][1];
                dis_2.push(x * x + y * y);
            }
        }

        dis_2.sort_unstable();

        dis_2[0] != 0 && dis_2[0] == dis_2[3] && dis_2[4] == dis_2[5]
    }
}
