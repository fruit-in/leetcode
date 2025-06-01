impl Solution {
    pub fn unhappy_friends(n: i32, preferences: Vec<Vec<i32>>, pairs: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut rank = vec![vec![n; n]; n];
        let mut is_unhappy = vec![false; n];

        for i in 0..n {
            for j in 0..n - 1 {
                rank[i][preferences[i][j] as usize] = j;
            }
        }

        for i in 0..n / 2 {
            let (x, y) = (pairs[i][0] as usize, pairs[i][1] as usize);

            for j in 0..n / 2 {
                if is_unhappy[x] && is_unhappy[y] {
                    break;
                }

                let (u, v) = (pairs[j][0] as usize, pairs[j][1] as usize);

                is_unhappy[x] |= rank[x][u] < rank[x][y] && rank[u][x] < rank[u][v];
                is_unhappy[x] |= rank[x][v] < rank[x][y] && rank[v][x] < rank[v][u];
                is_unhappy[y] |= rank[y][u] < rank[y][x] && rank[u][y] < rank[u][v];
                is_unhappy[y] |= rank[y][v] < rank[y][x] && rank[v][y] < rank[v][u];
            }
        }

        is_unhappy.iter().filter(|&&unhappy| unhappy).count() as i32
    }
}
