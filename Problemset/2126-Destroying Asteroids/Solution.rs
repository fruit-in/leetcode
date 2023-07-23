impl Solution {
    pub fn asteroids_destroyed(mass: i32, asteroids: Vec<i32>) -> bool {
        let mut mass = mass as i64;
        let mut asteroids = asteroids;
        asteroids.sort_unstable();

        for asteroid in asteroids {
            if mass >= asteroid as i64 {
                mass += asteroid as i64;
            } else {
                return false;
            }
        }

        true
    }
}
