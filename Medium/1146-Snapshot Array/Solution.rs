struct SnapshotArray {
    arrays: Vec<Vec<(i32, i32)>>,
    snap_id: i32,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SnapshotArray {

    fn new(length: i32) -> Self {
        Self {
            arrays: vec![vec![(0, 0)]; length as usize],
            snap_id: 0,
        }
    }

    fn set(&mut self, index: i32, val: i32) {
        if self.snap_id > self.arrays[index as usize].last().unwrap().0 {
            self.arrays[index as usize].push((self.snap_id, val));
        } else {
            self.arrays[index as usize].last_mut().unwrap().1 = val;
        }
    }

    fn snap(&mut self) -> i32 {
        self.snap_id += 1;

        self.snap_id - 1
    }

    fn get(&self, index: i32, snap_id: i32) -> i32 {
        match self.arrays[index as usize].binary_search_by_key(&snap_id, |&(a, b)| a) {
            Ok(i) => self.arrays[index as usize][i].1,
            Err(i) => self.arrays[index as usize][i - 1].1,
        }
    }
}

/**
 * Your SnapshotArray object will be instantiated and called as such:
 * let obj = SnapshotArray::new(length);
 * obj.set(index, val);
 * let ret_2: i32 = obj.snap();
 * let ret_3: i32 = obj.get(index, snap_id);
 */
