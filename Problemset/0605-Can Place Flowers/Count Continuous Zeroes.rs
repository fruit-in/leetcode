impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let mut n = n;
        let mut zeroes = 1;

        for i in flowerbed {
            if i == 0 {
                zeroes += 1;
            } else {
                n -= (zeroes - 1) / 2;
                zeroes = 0;
            }
        }

        zeroes += 1;
        n -= (zeroes - 1) / 2;

        n <= 0
    }
}
