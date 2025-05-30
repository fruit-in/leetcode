impl Solution {
    pub fn reaching_points(sx: i32, sy: i32, tx: i32, ty: i32) -> bool {
        let mut tx = tx;
        let mut ty = ty;

        while tx > sx && ty > sy {
            if tx < ty {
                ty -= tx;
            } else {
                tx -= ty;
            }
        }

        if tx < sx || ty < sy {
            false
        } else if tx == sx && ty == sy {
            true
        } else if tx == sx {
            ty > tx && (ty - sy) % sx == 0
        } else {
            tx > ty && (tx - sx) % sy == 0
        }
    }
}
