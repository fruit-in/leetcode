impl Solution {
    pub fn maximum_time(time: String) -> String {
        let mut time = time.into_bytes();

        match (time[0], time[1]) {
            (b'?', b'?') => {
                time[0] = b'2';
                time[1] = b'3';
            }
            (b'?', t1) if t1 < b'4' => time[0] = b'2',
            (b'?', t1) => time[0] = b'1',
            (b'2', b'?') => time[1] = b'3',
            (t0, b'?') => time[1] = b'9',
            (t0, t1) => (),
        }
        if time[3] == b'?' {
            time[3] = b'5';
        }
        if time[4] == b'?' {
            time[4] = b'9';
        }

        String::from_utf8(time).unwrap()
    }
}
