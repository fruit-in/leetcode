impl Solution {
    pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
        let mut people = names.iter().zip(heights.iter()).collect::<Vec<_>>();
        people.sort_unstable_by_key(|(_, &h)| -h);

        people.into_iter().map(|(n, _)| n.clone()).collect()
    }
}
