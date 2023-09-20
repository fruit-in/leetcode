use std::collections::HashMap;

impl Solution {
    pub fn invalid_transactions(transactions: Vec<String>) -> Vec<String> {
        let mut ret = vec![];

        for i in 0..transactions.len() {
            let transaction0 = transactions[i].split(',').collect::<Vec<_>>();
            let name0 = transaction0[0];
            let time0 = transaction0[1].parse::<i32>().unwrap();
            let amount0 = transaction0[2].parse::<i32>().unwrap();
            let city0 = transaction0[3];

            if amount0 > 1000 {
                ret.push(transactions[i].clone());
                continue;
            }

            for j in 0..transactions.len() {
                if j != i {
                    let transaction1 = transactions[j].split(',').collect::<Vec<_>>();
                    let name1 = transaction1[0];
                    let time1 = transaction1[1].parse::<i32>().unwrap();
                    let amount1 = transaction1[2].parse::<i32>().unwrap();
                    let city1 = transaction1[3];

                    if name0 == name1 && (time0 - time1).abs() <= 60 && city0 != city1 {
                        ret.push(transactions[i].clone());
                        break;
                    }
                }
            }
        }

        ret
    }
}
