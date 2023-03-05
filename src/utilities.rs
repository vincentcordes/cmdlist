// Credit to some Stack Overflow guru
// This function was helpful to understand types at first
pub fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

pub fn trim_quotes(str: String) -> String {
    let len = str.len() - 1;
    let mut trimmed = str.clone();
    trimmed.remove(len);
    trimmed.remove(0);
    return trimmed;
}
