impl Solution {
    pub fn find_content_children(g: Vec<i32>, s: Vec<i32>) -> i32 {
        let mut g = g;
        let mut s = s;
        g.sort_unstable();
        s.sort_unstable();

        let mut i = 0;
        let mut j = 0;
        let mut ret = 0;

        while i < g.len() && j < s.len() {
            if s[j] >= g[i] {
                ret += 1;
                i += 1;
            }
            j += 1;
        }

        ret
    }
}
