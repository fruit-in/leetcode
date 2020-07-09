# 1396. Design Underground System
Implement the class `UndergroundSystem` that supports three methods:
1. `checkIn(int id, string stationName, int t)`
* A customer with id card equal to `id`, gets in the station `stationName` at time `t`.
* A customer can only be checked into one place at a time.
2. `checkOut(int id, string stationName, int t)`
* A customer with id card equal to `id`, gets out from the station `stationName` at time `t`.
3. `getAverageTime(string startStation, string endStation)`
* Returns the average time to travel between the `startStation` and the `endStation`.
* The average time is computed from all the previous traveling from `startStation` to `endStation` that happened **directly**.
* Call to `getAverageTime` is always valid.

You can assume all calls to `checkIn` and `checkOut` methods are consistent. That is, if a customer gets in at time <b>t<sub>1</sub></b> at some station, then it gets out at time <b>t<sub>2</sub></b> with <b>t<sub>2</sub></b> > <b>t<sub>1</sub></b>. All events happen in chronological order.

#### Example 1:
<pre>
<strong>Input:</strong>
["UndergroundSystem","checkIn","checkIn","checkIn","checkOut","checkOut","checkOut","getAverageTime","getAverageTime","checkIn","getAverageTime","checkOut","getAverageTime"]
[[],[45,"Leyton",3],[32,"Paradise",8],[27,"Leyton",10],[45,"Waterloo",15],[27,"Waterloo",20],[32,"Cambridge",22],["Paradise","Cambridge"],["Leyton","Waterloo"],[10,"Leyton",24],["Leyton","Waterloo"],[10,"Waterloo",38],["Leyton","Waterloo"]]
<strong>Output:</strong>
[null,null,null,null,null,null,null,14.00000,11.00000,null,11.00000,null,12.00000]
<strong>Explanation:</strong>
UndergroundSystem undergroundSystem = new UndergroundSystem();
undergroundSystem.checkIn(45, "Leyton", 3);
undergroundSystem.checkIn(32, "Paradise", 8);
undergroundSystem.checkIn(27, "Leyton", 10);
undergroundSystem.checkOut(45, "Waterloo", 15);
undergroundSystem.checkOut(27, "Waterloo", 20);
undergroundSystem.checkOut(32, "Cambridge", 22);
undergroundSystem.getAverageTime("Paradise", "Cambridge");       // return 14.00000. There was only one travel from "Paradise" (at time 8) to "Cambridge" (at time 22)
undergroundSystem.getAverageTime("Leyton", "Waterloo");          // return 11.00000. There were two travels from "Leyton" to "Waterloo", a customer with id=45 from time=3 to time=15 and a customer with id=27 from time=10 to time=20. So the average time is ( (15-3) + (20-10) ) / 2 = 11.00000
undergroundSystem.checkIn(10, "Leyton", 24);
undergroundSystem.getAverageTime("Leyton", "Waterloo");          // return 11.00000
undergroundSystem.checkOut(10, "Waterloo", 38);
undergroundSystem.getAverageTime("Leyton", "Waterloo");          // return 12.00000
</pre>

#### Example 2:
<pre>
<strong>Input:</strong>
["UndergroundSystem","checkIn","checkOut","getAverageTime","checkIn","checkOut","getAverageTime","checkIn","checkOut","getAverageTime"]
[[],[10,"Leyton",3],[10,"Paradise",8],["Leyton","Paradise"],[5,"Leyton",10],[5,"Paradise",16],["Leyton","Paradise"],[2,"Leyton",21],[2,"Paradise",30],["Leyton","Paradise"]]
<strong>Output:</strong>
[null,null,null,5.00000,null,null,5.50000,null,null,6.66667]
<strong>Explanation:</strong>
UndergroundSystem undergroundSystem = new UndergroundSystem();
undergroundSystem.checkIn(10, "Leyton", 3);
undergroundSystem.checkOut(10, "Paradise", 8);
undergroundSystem.getAverageTime("Leyton", "Paradise"); // return 5.00000
undergroundSystem.checkIn(5, "Leyton", 10);
undergroundSystem.checkOut(5, "Paradise", 16);
undergroundSystem.getAverageTime("Leyton", "Paradise"); // return 5.50000
undergroundSystem.checkIn(2, "Leyton", 21);
undergroundSystem.checkOut(2, "Paradise", 30);
undergroundSystem.getAverageTime("Leyton", "Paradise"); // return 6.66667
</pre>

#### Constraints:
* There will be at most `20000` operations.
* `1 <= id, t <= 10^6`
* All strings consist of uppercase, lowercase English letters and digits.
* `1 <= stationName.length <= 10`
* Answers within `10^-5` of the actual value will be accepted as correct.

## Solutions (Rust)

### 1. HashMap
```Rust
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
```
