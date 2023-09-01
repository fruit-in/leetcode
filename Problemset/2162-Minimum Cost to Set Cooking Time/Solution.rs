impl Solution {
    pub fn min_cost_set_time(
        start_at: i32,
        move_cost: i32,
        push_cost: i32,
        target_seconds: i32,
    ) -> i32 {
        let mut ret = i32::MAX;

        for m in (target_seconds - 40).max(0) / 60..=(target_seconds / 60).min(99) {
            let s = target_seconds - m * 60;
            let mut curr_at = start_at;
            let mut cost = 0;
            let mut flag = false;

            for x in [m / 10, m % 10, s / 10, s % 10] {
                flag |= x > 0;
                if flag {
                    if curr_at != x {
                        curr_at = x;
                        cost += move_cost;
                    }
                    cost += push_cost;
                }
            }

            ret = ret.min(cost);
        }

        ret
    }
}
