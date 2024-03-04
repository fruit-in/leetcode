use std::collections::HashMap;

impl Solution {
    pub fn is_rectangle_cover(rectangles: Vec<Vec<i32>>) -> bool {
        let (mut minx, mut miny, mut maxa, mut maxb) = (i32::MAX, i32::MAX, i32::MIN, i32::MIN);
        let mut area = 0;
        let mut count = HashMap::new();

        for rect in &rectangles {
            let (x, y, a, b) = (rect[0], rect[1], rect[2], rect[3]);
            minx = minx.min(x);
            miny = miny.min(y);
            maxa = maxa.max(a);
            maxb = maxb.max(b);
            area += (a - x) as i64 * (b - y) as i64;
            *count.entry((x, y)).or_insert(0) += 1;
            *count.entry((x, b)).or_insert(0) += 1;
            *count.entry((a, y)).or_insert(0) += 1;
            *count.entry((a, b)).or_insert(0) += 1;
        }

        if (maxa - minx) as i64 * (maxb - miny) as i64 != area {
            return false;
        }

        for (&(x, y), &c) in count.iter() {
            if (x == minx || x == maxa) && (y == miny || y == maxb) && c != 1 {
                return false;
            } else if ((x != minx && x != maxa) || (y != miny && y != maxb)) && c != 2 && c != 4 {
                return false;
            }
        }

        true
    }
}
