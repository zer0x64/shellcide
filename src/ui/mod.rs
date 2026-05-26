pub mod controls;
pub mod editor;
pub mod header;
pub mod instructions;
pub mod memory;
pub mod registers;
pub mod struct_packer;
pub mod syscalls;
pub mod theme;

#[cfg(test)]
mod tests;

#[derive(PartialEq, Clone, Copy, Debug)]
pub(crate) enum LeftBottomTab {
    Syscalls,
    StructPacker,
    Instructions,
}

pub(crate) fn contains_case_insensitive(haystack: &str, needle: &str) -> bool {
    if needle.is_empty() {
        return true;
    }
    haystack
        .as_bytes()
        .windows(needle.len())
        .any(|window| window.eq_ignore_ascii_case(needle.as_bytes()))
}

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
        clean
            .parse::<u64>()
            .ok()
            .or_else(|| u64::from_str_radix(clean, 16).ok())
    }
}
