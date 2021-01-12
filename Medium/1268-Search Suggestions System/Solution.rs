impl Solution {
    pub fn suggested_products(mut products: Vec<String>, search_word: String) -> Vec<Vec<String>> {
        let mut ret = vec![];

        products.sort_unstable();

        for i in 1..=search_word.len() {
            products.retain(|s| s.starts_with(search_word.get(0..i).unwrap()));
            ret.push(products.iter().take(3).cloned().collect());
        }

        ret
    }
}
