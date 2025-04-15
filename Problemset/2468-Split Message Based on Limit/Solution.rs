impl Solution {
    fn splitn(n: usize, mut length: usize, limit: usize) -> Option<usize> {
        let ceil = 10_usize.pow(n as u32);
        let max_index_size = 3 + n * 2;

        if limit <= max_index_size {
            return None;
        }

        length += (3 + n) * (ceil / 10 - 1);
        length += (1..n)
            .map(|x| x * 9 * 10_usize.pow(x as u32 - 1))
            .sum::<usize>();
        length = length.saturating_sub(limit * (ceil / 10 - 1));

        if length == 0
            || (length + limit - max_index_size - 1) / (limit - max_index_size) + ceil / 10 - 1
                >= ceil
        {
            return None;
        }

        Some((length + limit - max_index_size - 1) / (limit - max_index_size) + ceil / 10 - 1)
    }

    pub fn split_message(message: String, limit: i32) -> Vec<String> {
        let limit = limit as usize;
        let length = message.len();
        let mut b = 0;
        let mut start = 0;
        let mut ret = vec![];

        for n in 1..5 {
            if let Some(x) = Self::splitn(n, length, limit) {
                b = x;
                break;
            }
        }

        for a in 1..=b {
            let index = format!("<{}/{}>", a, b);
            let end = length.min(start + limit - index.len());

            ret.push(format!(
                "{}{}",
                message.get(start..end).unwrap_or(""),
                index
            ));
            start = end;
        }

        ret
    }
}
