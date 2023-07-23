impl Solution {
    pub fn minimum_recolors(blocks: String, k: i32) -> i32 {
        let k = k as usize;
        let mut count_w = blocks.chars().take(k).filter(|&c| c == 'W').count();
        let blocks = blocks.as_bytes();
        let mut ret = count_w;

        for i in k..blocks.len() {
            count_w += (blocks[i] == b'W') as usize - (blocks[i - k] == b'W') as usize;
            ret = ret.min(count_w);
        }

        ret as i32
    }
}
