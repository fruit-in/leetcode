impl Solution {
    pub fn minimum_refill(plants: Vec<i32>, capacity_a: i32, capacity_b: i32) -> i32 {
        let n = plants.len();
        let mut water_a = capacity_a;
        let mut water_b = capacity_b;
        let mut ret = 0;

        for i in 0..n / 2 {
            if water_a < plants[i] {
                water_a = capacity_a;
                ret += 1;
            }
            if water_b < plants[n - 1 - i] {
                water_b = capacity_b;
                ret += 1;
            }

            water_a -= plants[i];
            water_b -= plants[n - 1 - i];
        }

        ret + (n % 2 == 1 && water_a.max(water_b) < plants[n / 2]) as i32
    }
}
