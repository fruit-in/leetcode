impl Solution {
    pub fn video_stitching(clips: Vec<Vec<i32>>, t: i32) -> i32 {
        let mut clips = clips;
        let mut start = 0;
        let mut ret = 0;

        clips.sort_unstable();

        for i in 1..clips.len() {
            if clips[i][1] < clips[i - 1][1] {
                clips[i][1] = clips[i - 1][1];
            }
        }

        while start < t {
            match clips.binary_search(&vec![start, i32::MAX]) {
                Err(i) if i > 0 => {
                    if start >= clips[i - 1][1] {
                        return -1;
                    }

                    start = clips[i - 1][1];
                    ret += 1;
                }
                _ => return -1,
            }
        }

        ret
    }
}
