impl Solution {
    pub fn entity_parser(text: String) -> String {
        text.replace("&quot;", "\"")
            .replace("&apos;", "'")
            .replace("&gt;", ">")
            .replace("&lt;", "<")
            .replace("&frasl;", "/")
            .replace("&amp;", "&")
    }
}
