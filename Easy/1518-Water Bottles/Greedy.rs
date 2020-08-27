impl Solution {
    pub fn num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32 {
        let mut num_empty = num_bottles;
        let mut ret = num_bottles;

        while num_empty >= num_exchange {
            ret += num_empty / num_exchange;
            num_empty = num_empty % num_exchange + num_empty / num_exchange;
        }

        ret
    }
}
