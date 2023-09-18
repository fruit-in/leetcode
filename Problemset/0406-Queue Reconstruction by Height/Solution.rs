impl Solution {
    pub fn reconstruct_queue(people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut people = people;
        let mut queue = vec![];

        people.sort_unstable_by_key(|p| (-p[0], p[1]));

        for p in people {
            queue.insert(p[1] as usize, p);
        }

        queue
    }
}
