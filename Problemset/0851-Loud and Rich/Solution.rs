impl Solution {
    pub fn loud_and_rich(richer: Vec<Vec<i32>>, quiet: Vec<i32>) -> Vec<i32> {
        let n = quiet.len();
        let mut richer_count = vec![0; n];
        let mut poorer_people = vec![vec![]; n];
        let mut people = vec![];
        let mut answer = (0..n as i32).collect::<Vec<_>>();

        for pair in &richer {
            richer_count[pair[1] as usize] += 1;
            poorer_people[pair[0] as usize].push(pair[1] as usize);
        }

        for i in 0..n {
            if richer_count[i] == 0 {
                people.push(i);
            }
        }

        while let Some(x) = people.pop() {
            for &y in &poorer_people[x] {
                richer_count[y] -= 1;
                if richer_count[y] == 0 {
                    people.push(y);
                }
                if quiet[answer[x] as usize] < quiet[answer[y] as usize] {
                    answer[y] = answer[x];
                }
            }
        }

        answer
    }
}
