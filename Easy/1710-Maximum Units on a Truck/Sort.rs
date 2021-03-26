impl Solution {
    pub fn maximum_units(mut box_types: Vec<Vec<i32>>, mut truck_size: i32) -> i32 {
        box_types.sort_unstable_by_key(|b| b[1]);
        let mut ret = 0;

        while let Some(b) = box_types.pop() {
            ret += b[0].min(truck_size) * b[1];
            truck_size -= b[0];
            if truck_size <= 0 {
                break;
            }
        }

        ret
    }
}
