pub(crate) fn build_bulk(content: String) -> String {
    format!("${}\r\n{}\r\n", content.len(), content)
}

pub(crate) fn build_array(content: Vec<String>) -> String {
    // let lines_count = content.chars().filter(|c| *c == '$').count();
    let lines_count = content.len();

    format!("*{}\r\n{}", lines_count, content.join(""))
}
