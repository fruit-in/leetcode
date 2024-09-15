use std::collections::HashSet;

impl Solution {
    pub fn closest_cost(base_costs: Vec<i32>, topping_costs: Vec<i32>, target: i32) -> i32 {
        let mut costs = HashSet::from([0]);
        let mut ret = i32::MAX;

        for &topping in &topping_costs {
            for &cost in costs.clone().iter() {
                costs.insert(cost + topping);
                costs.insert(cost + topping + topping);
            }
        }

        for &cost in costs.iter() {
            for i in 0..base_costs.len() {
                let x = cost + base_costs[i];

                if (x - target).abs() < (ret - target).abs() {
                    ret = x;
                } else if (x - target).abs() == (ret - target).abs() {
                    ret = ret.min(x);
                }
            }
        }

        ret
    }
}
