pub mod theme;
pub mod editor;
pub mod registers;
pub mod memory;
pub mod syscalls;
pub mod controls;
pub mod header;
pub mod struct_packer;

#[cfg(test)]
mod tests;

pub(crate) fn parse_u64_input(input: &str) -> Option<u64> {
    let clean = input.trim();
    if clean.is_empty() {
        return None;
    }
    let lowercase = clean.to_lowercase();
    if lowercase.starts_with("0x") {
        u64::from_str_radix(&clean[2..], 16).ok()
    } else if lowercase.ends_with('h') {
        u64::from_str_radix(&clean[..clean.len() - 1], 16).ok()
    } else {
        clean.parse::<u64>().ok().or_else(|| u64::from_str_radix(clean, 16).ok())
    }
}


