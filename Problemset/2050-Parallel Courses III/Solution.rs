impl Solution {
    pub fn minimum_time(n: i32, relations: Vec<Vec<i32>>, time: Vec<i32>) -> i32 {
        let mut next_courses = vec![vec![]; n as usize];
        let mut indgree = vec![0; n as usize];
        let mut start = vec![0; n as usize];
        let mut stack = vec![];
        let mut ret = 0;

        for relation in &relations {
            let (prev, next) = (relation[0] as usize - 1, relation[1] as usize - 1);

            next_courses[prev].push(next);
            indgree[next] += 1;
        }

        for i in 0..n as usize {
            if indgree[i] == 0 {
                stack.push(i);
            }
        }

        while let Some(prev) = stack.pop() {
            let end = start[prev] + time[prev];

            for &next in &next_courses[prev] {
                indgree[next] -= 1;
                start[next] = start[next].max(end);
                if indgree[next] == 0 {
                    stack.push(next);
                }
            }

            ret = ret.max(end);
        }

        ret
    }
}
