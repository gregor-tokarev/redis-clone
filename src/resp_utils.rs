pub(crate) fn build_bulk(content: String) -> String {
    format!("${}\r\n{}\r\n", content.len(), content)
}

pub(crate) fn build_array(content: String) -> String {
    let lines_count = content.chars().filter(|c| *c == '$').count();

    format!("*{}\r\n{}", lines_count, content)
}
