impl Solution {
    pub fn corp_flight_bookings(bookings: Vec<Vec<i32>>, n: i32) -> Vec<i32> {
        let mut ret = vec![0; n as usize];

        for booking in bookings {
            if booking[0] > 1 {
                ret[booking[0] as usize - 2] -= booking[2];
            }
            ret[booking[1] as usize - 1] += booking[2];
        }

        for i in (0..(n as usize - 1)).rev() {
            ret[i] += ret[i + 1];
        }

        ret
    }
}
