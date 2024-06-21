pub fn extract_summary(content: &str, max_length: usize) -> String {
    if content.len() <= max_length {
        content.to_string()
    } else {
        let summary: String = content.chars().take(max_length).collect();
        summary + "..."
    }
}
