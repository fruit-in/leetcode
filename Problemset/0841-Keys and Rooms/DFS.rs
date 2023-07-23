impl Solution {
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        let mut opened = vec![false; rooms.len()];
        let mut keys = vec![0];

        while let Some(curr) = keys.pop() {
            if !opened[curr] {
                opened[curr] = true;
                for &key in &rooms[curr] {
                    if !opened[key as usize] {
                        keys.push(key as usize);
                    }
                }
            }
        }

        opened.iter().all(|&room| room)
    }
}
