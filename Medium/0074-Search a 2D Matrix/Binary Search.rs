impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if matrix.len() == 0 {
            return false;
        }

        let m = matrix.len();
        let n = matrix[0].len();
        let mut l = 0;
        let mut r = m * n;

        while l < r {
            let mid = (l + r) / 2;
            let row = mid / n;
            let col = mid % n;

            if target == matrix[row][col] {
                return true;
            } else if target < matrix[row][col] {
                r = mid;
            } else {
                l = mid + 1;
            }
        }

        false
    }
}
