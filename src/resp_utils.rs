pub(crate) fn build_bulk(content: String) -> String {
    format!("${}\r\n{}\r\n", content.len(), content)
}
