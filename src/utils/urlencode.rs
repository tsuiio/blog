pub trait URLEncode {
    fn encode(&self) -> String;
}

impl<T: AsRef<str>> URLEncode for T {
    fn encode(&self) -> String {
        let input = self.as_ref();
        let mut encoded = String::new();
        for c in input.chars() {
            match c {
                '0'..='9' | 'a'..='z' | 'A'..='Z' => encoded.push(c),
                _ => encoded.push_str(&format!("%{:02X}", c as u32)),
            }
        }
        encoded
    }
}
