/// Changes "hello" into hello
/// Does not change "hello or "hello
pub(crate) fn trim_surrounding_quotes(mut str: String) -> String {
    if str.ends_with("\"") && str.starts_with("\"") {
        // Assumes string literal is surrounded by " ";
        str.truncate(str.len() - 1);
        return str[1..].trim().to_string();
    }
    str
}
