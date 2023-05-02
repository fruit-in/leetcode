impl Solution {
    pub fn min_characters(a: String, b: String) -> i32 {
        let mut ret = a
            .chars()
            .chain(b.chars())
            .filter(|&chab| chab != 'a')
            .count();

        for ch in 'b'..='z' {
            ret = ret.min(
                a.chars().filter(|&cha| cha >= ch).count()
                    + b.chars().filter(|&chb| chb < ch).count(),
            );
            ret = ret.min(
                a.chars().filter(|&cha| cha < ch).count()
                    + b.chars().filter(|&chb| chb >= ch).count(),
            );
            ret = ret.min(
                a.chars()
                    .chain(b.chars())
                    .filter(|&chab| chab != ch)
                    .count(),
            );
        }

        ret as i32
    }
}
