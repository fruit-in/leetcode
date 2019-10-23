impl Solution {
    pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
        let mut max_area = 0_f64;

        for i in 0..(points.len() - 2) {
            let a = &points[i];
            for j in (i + 1)..(points.len() - 1) {
                let b = &points[j];
                for k in (j + 1)..points.len() {
                    let c = &points[k];
                    max_area = max_area.max(Self::area(a, b, c));
                }
            }
        }

        max_area
    }
    
    pub fn area(a: &Vec<i32>, b: &Vec<i32>, c: &Vec<i32>) -> f64 {
        (a[0] * b[1] + b[0] * c[1] + c[0] * a[1] -
        a[1] * b[0] - b[1] * c[0] - c[1] * a[0]).abs() as f64 / 2.0
    }
}
