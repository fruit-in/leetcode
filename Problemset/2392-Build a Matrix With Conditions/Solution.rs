impl Solution {
    pub fn build_matrix(
        k: i32,
        row_conditions: Vec<Vec<i32>>,
        col_conditions: Vec<Vec<i32>>,
    ) -> Vec<Vec<i32>> {
        let k = k as usize;
        let mut belows = vec![vec![]; k + 1];
        let mut rights = vec![vec![]; k + 1];
        let mut indegree = vec![0; k + 1];
        let mut stack = vec![];
        let mut r = 0;
        let mut c = 0;
        let mut row = vec![k; k + 1];
        let mut col = vec![k; k + 1];
        let mut ret = vec![vec![0; k]; k];

        for condition in &row_conditions {
            let (above, below) = (condition[0] as usize, condition[1] as usize);
            belows[above].push(below);
            indegree[below] += 1;
        }

        stack = (1..=k).filter(|&x| indegree[x] == 0).collect();

        while let Some(above) = stack.pop() {
            row[above] = r;
            r += 1;
            for &below in &belows[above] {
                indegree[below] -= 1;
                if indegree[below] == 0 {
                    stack.push(below);
                }
            }
        }

        if r != k {
            return vec![];
        }

        indegree = vec![0; k + 1];

        for condition in &col_conditions {
            let (left, right) = (condition[0] as usize, condition[1] as usize);
            rights[left].push(right);
            indegree[right] += 1;
        }

        stack = (1..=k).filter(|&x| indegree[x] == 0).collect();

        while let Some(left) = stack.pop() {
            col[left] = c;
            c += 1;
            for &right in &rights[left] {
                indegree[right] -= 1;
                if indegree[right] == 0 {
                    stack.push(right);
                }
            }
        }

        if c != k {
            return vec![];
        }

        for x in 1..=k {
            ret[row[x]][col[x]] = x as i32;
        }

        ret
    }
}
