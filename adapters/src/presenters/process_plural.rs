pub(super) fn process_plural(count: &u32, singular: &str, plural: &str) -> String {
    match count {
        1 => format!("{} {}", count, singular),
        _ => format!("{} {}", count, plural),
    }
}
