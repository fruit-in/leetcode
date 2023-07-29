impl Solution {
    pub fn min_operations_max_profit(
        customers: Vec<i32>,
        boarding_cost: i32,
        running_cost: i32,
    ) -> i32 {
        if 4 * boarding_cost <= running_cost {
            return -1;
        }

        let mut i = 0;
        let mut customer = 0;
        let mut profit = 0;
        let mut max_profit = 0;
        let mut ret = -1;

        while i < customers.len() || customer > 0 {
            customer += customers.get(i).unwrap_or(&0);
            profit += customer.min(4) * boarding_cost - running_cost;
            customer -= customer.min(4);
            i += 1;

            if profit > max_profit {
                max_profit = profit;
                ret = i as i32;
            }
        }

        ret
    }
}
