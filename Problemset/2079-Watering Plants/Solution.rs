impl Solution {
    pub fn watering_plants(plants: Vec<i32>, capacity: i32) -> i32 {
        let mut water = capacity;
        let mut ret = 0;

        for i in 0..plants.len() {
            if water < plants[i] {
                water = capacity;
                ret += 2 * i;
            }
            water -= plants[i];
            ret += 1;
        }

        ret as i32
    }
}
