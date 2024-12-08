pub fn format_number(value: f64) -> String {
    if value == value.trunc() {
        format!("{:.1}", value)
    } else {
        format!("{}", value)
    }
}
