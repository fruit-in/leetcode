impl Solution {
    pub fn remove_subfolders(folder: Vec<String>) -> Vec<String> {
        let mut folder = folder.into_iter().map(|s| s + "/").collect::<Vec<_>>();
        folder.sort_unstable();
        let mut ret = Vec::new();

        for f in folder {
            if ret.is_empty() || !f.starts_with(ret.last().unwrap()) {
                ret.push(f);
            }
        }
        ret.iter_mut().for_each(|s| {
            s.pop();
        });

        ret
    }
}
