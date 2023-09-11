use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn get_number_of_backlog_orders(orders: Vec<Vec<i32>>) -> i32 {
        let mut buys = BinaryHeap::new();
        let mut sells = BinaryHeap::new();

        for order in &orders {
            let price = order[0];
            let mut amount = order[1];
            let order_type = order[2];

            if order_type == 0 {
                while let Some(Reverse((p, a))) = sells.pop() {
                    if p > price {
                        sells.push(Reverse((p, a)));
                        break;
                    } else if amount >= a {
                        amount -= a;
                    } else {
                        sells.push(Reverse((p, a - amount)));
                        amount = 0;
                        break;
                    }
                }

                if amount > 0 {
                    buys.push((price, amount));
                }
            } else {
                while let Some((p, a)) = buys.pop() {
                    if p < price {
                        buys.push((p, a));
                        break;
                    } else if amount >= a {
                        amount -= a;
                    } else {
                        buys.push((p, a - amount));
                        amount = 0;
                        break;
                    }
                }

                if amount > 0 {
                    sells.push(Reverse((price, amount)));
                }
            }
        }

        (buys
            .iter()
            .map(|&(_, a)| a as i64)
            .chain(sells.iter().map(|&Reverse((_, a))| a as i64))
            .sum::<i64>()
            % 1_000_000_007) as i32
    }
}
