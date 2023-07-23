impl Solution {
    pub fn mem_leak(memory1: i32, memory2: i32) -> Vec<i32> {
        let mut bits = 1;
        let mut crash_time = 1;
        let mut memory1 = memory1;
        let mut memory2 = memory2;

        while memory1.max(memory2) >= bits {
            crash_time += 1;
            if memory1 >= memory2 {
                memory1 -= bits;
            } else {
                memory2 -= bits;
            }
            bits += 1;
        }

        vec![crash_time, memory1, memory2]
    }
}
