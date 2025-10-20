use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn contain_virus(mut is_infected: Vec<Vec<i32>>) -> i32 {
        let m = is_infected.len();
        let n = is_infected[0].len();
        let mut walls = HashSet::new();

        loop {
            let mut visited = HashSet::new();
            let mut spreads = HashMap::new();
            let mut areas = HashMap::new();
            let mut needs = HashMap::new();
            let mut max_threat_level = 0;
            let mut max_threat_area = (0, 0);

            for i in 0..m {
                for j in 0..n {
                    if is_infected[i][j] != 1 || visited.contains(&(i, j)) {
                        continue;
                    }

                    let mut stack = vec![(i, j)];
                    visited.insert((i, j));
                    areas.insert((i, j), vec![(i, j)]);
                    spreads.insert((i, j), HashSet::new());
                    needs.insert((i, j), HashSet::new());

                    while let Some((r, c)) = stack.pop() {
                        if r > 0 {
                            if is_infected[r - 1][c] == 1 && !visited.contains(&(r - 1, c)) {
                                stack.push((r - 1, c));
                                visited.insert((r - 1, c));
                                areas.get_mut(&(i, j)).unwrap().push((r - 1, c));
                            } else if is_infected[r - 1][c] == 0
                                && !walls.contains(&(r - 1, c, 'D'))
                            {
                                spreads.get_mut(&(i, j)).unwrap().insert((r - 1, c));
                                needs.get_mut(&(i, j)).unwrap().insert((r - 1, c, 'D'));
                            }
                        }
                        if r < m - 1 {
                            if is_infected[r + 1][c] == 1 && !visited.contains(&(r + 1, c)) {
                                stack.push((r + 1, c));
                                visited.insert((r + 1, c));
                                areas.get_mut(&(i, j)).unwrap().push((r + 1, c));
                            } else if is_infected[r + 1][c] == 0 && !walls.contains(&(r, c, 'D')) {
                                spreads.get_mut(&(i, j)).unwrap().insert((r + 1, c));
                                needs.get_mut(&(i, j)).unwrap().insert((r, c, 'D'));
                            }
                        }
                        if c > 0 {
                            if is_infected[r][c - 1] == 1 && !visited.contains(&(r, c - 1)) {
                                stack.push((r, c - 1));
                                visited.insert((r, c - 1));
                                areas.get_mut(&(i, j)).unwrap().push((r, c - 1));
                            } else if is_infected[r][c - 1] == 0
                                && !walls.contains(&(r, c - 1, 'R'))
                            {
                                spreads.get_mut(&(i, j)).unwrap().insert((r, c - 1));
                                needs.get_mut(&(i, j)).unwrap().insert((r, c - 1, 'R'));
                            }
                        }
                        if c < n - 1 {
                            if is_infected[r][c + 1] == 1 && !visited.contains(&(r, c + 1)) {
                                stack.push((r, c + 1));
                                visited.insert((r, c + 1));
                                areas.get_mut(&(i, j)).unwrap().push((r, c + 1));
                            } else if is_infected[r][c + 1] == 0 && !walls.contains(&(r, c, 'R')) {
                                spreads.get_mut(&(i, j)).unwrap().insert((r, c + 1));
                                needs.get_mut(&(i, j)).unwrap().insert((r, c, 'R'));
                            }
                        }
                    }

                    if spreads[&(i, j)].len() > max_threat_level {
                        max_threat_level = spreads[&(i, j)].len();
                        max_threat_area = (i, j);
                    }
                }
            }

            if max_threat_level == 0 {
                break;
            }

            for (&(r, c), s) in spreads.iter() {
                if (r, c) != max_threat_area {
                    for &(r, c) in s {
                        is_infected[r][c] = 1;
                    }
                }
            }

            for &(r, c) in &areas[&max_threat_area] {
                is_infected[r][c] = -1;
            }

            for &(r, c, d) in &needs[&max_threat_area] {
                walls.insert((r, c, d));
            }
        }

        walls.len() as i32
    }
}
