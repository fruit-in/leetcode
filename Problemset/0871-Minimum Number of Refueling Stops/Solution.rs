use std::collections::BinaryHeap;

impl Solution {
    pub fn min_refuel_stops(target: i32, start_fuel: i32, stations: Vec<Vec<i32>>) -> i32 {
        let mut fuel = start_fuel;
        let mut position = 0;
        let mut fuels = BinaryHeap::new();

        for station in &stations {
            while position + fuel < station[0] {
                match fuels.pop() {
                    Some(f) => fuel += f,
                    None => return -1,
                }
            }

            fuel -= station[0] - position;
            position = station[0];
            fuels.push(station[1]);
        }

        while position + fuel < target {
            match fuels.pop() {
                Some(f) => fuel += f,
                None => return -1,
            }
        }

        (stations.len() - fuels.len()) as i32
    }
}
