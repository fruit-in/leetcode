struct Allocator {
    memory: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Allocator {
    fn new(n: i32) -> Self {
        Self {
            memory: vec![0; n as usize],
        }
    }

    fn allocate(&mut self, size: i32, m_id: i32) -> i32 {
        let size = size as usize;
        let mut count = self.memory.iter().take(size).filter(|&&x| x == 0).count();

        for i in 0..=self.memory.len().saturating_sub(size) {
            if count == size {
                for j in 0..size {
                    self.memory[i + j] = m_id;
                }

                return i as i32;
            }

            count -= (self.memory[i] == 0) as usize;
            count += (*self.memory.get(i + size).unwrap_or(&1) == 0) as usize;
        }

        -1
    }

    fn free(&mut self, m_id: i32) -> i32 {
        let mut count = 0;

        for i in 0..self.memory.len() {
            if self.memory[i] == m_id {
                self.memory[i] = 0;
                count += 1;
            }
        }

        count
    }
}

/**
 * Your Allocator object will be instantiated and called as such:
 * let obj = Allocator::new(n);
 * let ret_1: i32 = obj.allocate(size, mID);
 * let ret_2: i32 = obj.free(mID);
 */
