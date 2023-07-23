impl Solution {
    pub fn find_radius(houses: Vec<i32>, heaters: Vec<i32>) -> i32 {
        let mut houses = houses;
        let mut heaters = heaters;
        houses.sort_unstable();
        heaters.sort_unstable();

        let mut i = 0;
        let mut min_radius = 0;

        for j in 0..houses.len() {
            while i < heaters.len() - 1 && heaters[i + 1] < houses[j] {
                i += 1;
            }

            let mut min_distance = (houses[j] - heaters[i]).abs();
            if i < heaters.len() - 1 {
                min_distance = min_distance.min(heaters[i + 1] - houses[j]);
            }

            min_radius = min_radius.max(min_distance);
        }

        min_radius
    }
}
