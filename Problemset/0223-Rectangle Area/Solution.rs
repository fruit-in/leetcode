impl Solution {
    pub fn compute_area(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32, h: i32) -> i32 {
        let area0 = (c - a) * (d - b);
        let area1 = (g - e) * (h - f);
        let area2 = (c.min(g).saturating_sub(a.max(e))).max(0) *
            (d.min(h).saturating_sub(b.max(f))).max(0);

        area0 + area1 - area2
    }
}
