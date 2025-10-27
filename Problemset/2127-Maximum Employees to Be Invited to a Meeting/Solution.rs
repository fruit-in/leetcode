impl Solution {
    pub fn maximum_invitations(favorite: Vec<i32>) -> i32 {
        let favorite = favorite.iter().map(|&fav| fav as usize).collect::<Vec<_>>();
        let n = favorite.len();
        let mut indegree = vec![0; n];
        let mut stack = vec![];
        let mut incycle = vec![true; n];
        let mut chainlength = vec![1; n];
        let mut couplesum = 0;
        let mut cyclemax = 0;

        for i in 0..n {
            indegree[favorite[i]] += 1;
        }

        for i in 0..n {
            if indegree[i] == 0 {
                stack.push(i);
            }
        }

        while let Some(i) = stack.pop() {
            indegree[favorite[i]] -= 1;
            incycle[i] = false;
            chainlength[favorite[i]] = chainlength[favorite[i]].max(chainlength[i] + 1);
            if indegree[favorite[i]] == 0 {
                stack.push(favorite[i]);
            }
        }

        for i in 0..n {
            if incycle[i] {
                if favorite[favorite[i]] == i {
                    couplesum += chainlength[i];
                } else {
                    let mut j = favorite[i];
                    let mut length = 1;

                    while j != i {
                        incycle[j] = false;
                        j = favorite[j];
                        length += 1;
                    }

                    cyclemax = cyclemax.max(length);
                }
            }
        }

        couplesum.max(cyclemax)
    }
}
