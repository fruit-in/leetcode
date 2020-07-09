use std::collections::HashMap;

struct UndergroundSystem {
    still_in: HashMap<i32, (String, i32)>,
    times: HashMap<(String, String), (i32, i32)>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl UndergroundSystem {

    fn new() -> Self {
        Self {
            still_in: HashMap::new(),
            times: HashMap::new(),
        }
    }

    fn check_in(&mut self, id: i32, station_name: String, t: i32) {
        self.still_in.insert(id, (station_name, t));
    }

    fn check_out(&mut self, id: i32, station_name: String, t: i32) {
        let (start_name, start_t) = self.still_in.remove(&id).unwrap();
        let (time, cnt) = self.times.entry((start_name, station_name)).or_insert((0, 0));
        *time += t - start_t;
        *cnt += 1;
    }

    fn get_average_time(&self, start_station: String, end_station: String) -> f64 {
        let (time, cnt) = self.times.get(&(start_station, end_station)).unwrap();
        *time as f64 / *cnt as f64
    }
}

/**
 * Your UndergroundSystem object will be instantiated and called as such:
 * let obj = UndergroundSystem::new();
 * obj.check_in(id, stationName, t);
 * obj.check_out(id, stationName, t);
 * let ret_3: f64 = obj.get_average_time(startStation, endStation);
 */
