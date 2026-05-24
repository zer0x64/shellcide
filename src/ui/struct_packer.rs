use eframe::egui::{self, Color32};
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::app::ShellcideApp;

#[derive(PartialEq, Clone, Copy, Debug)]
pub(crate) enum LeftBottomTab {
    Syscalls,
    StructPacker,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub(crate) enum StructType {
    SockAddrIn,   // sockaddr_in (IPv4)
    SockAddrIn6,  // sockaddr_in6 (IPv6)
    SockAddrUn,   // sockaddr_un (Unix)
    SockAddr,     // sockaddr (Generic)
}

pub(crate) struct StructPackerState {
    pub(crate) selected_struct: StructType,
    // IPv4 fields
    pub(crate) ipv4_port: u16,
    pub(crate) ipv4_addr: String,
    // IPv6 fields
    pub(crate) ipv6_port: u16,
    pub(crate) ipv6_addr: String,
    pub(crate) ipv6_flowinfo: u32,
    pub(crate) ipv6_scope_id: u32,
    // Unix fields
    pub(crate) unix_path: String,
    pub(crate) unix_abstract: bool,
    // Generic fields
    pub(crate) generic_family: u16,
    pub(crate) generic_data: String,
}

impl Default for StructPackerState {
    fn default() -> Self {
        Self {
            selected_struct: StructType::SockAddrIn,
            ipv4_port: 4444,
            ipv4_addr: "127.0.0.1".to_string(),
            ipv6_port: 4444,
            ipv6_addr: "::1".to_string(),
            ipv6_flowinfo: 0,
            ipv6_scope_id: 0,
            unix_path: "/tmp/sock".to_string(),
            unix_abstract: false,
            generic_family: 2,
            generic_data: "7f 00 00 01".to_string(),
        }
    }
}

pub(crate) struct PackedResult {
    pub(crate) full_bytes: Vec<u8>,
    pub(crate) minimal_bytes: Vec<u8>,
    pub(crate) assembly_full: String,
    pub(crate) assembly_minimal: String,
}

impl StructPackerState {
    pub(crate) fn pack(&self, att_syntax: bool) -> Result<PackedResult, String> {
        match self.selected_struct {
            StructType::SockAddrIn => {
                let ip: Ipv4Addr = self.ipv4_addr.trim().parse()
                    .map_err(|e| format!("Invalid IPv4 address: {}", e))?;
                
                let port_bytes = self.ipv4_port.to_be_bytes();
                let ip_octets = ip.octets();
                
                let mut bytes = Vec::new();
                bytes.extend_from_slice(&(2u16).to_ne_bytes()); // AF_INET = 2
                bytes.extend_from_slice(&port_bytes);
                bytes.extend_from_slice(&ip_octets);
                bytes.extend_from_slice(&[0u8; 8]); // sin_zero

                let lines = vec![
                    (
                        "dw 2".to_string(),
                        ".short 2".to_string(),
                        "sin_family = AF_INET (2)".to_string(),
                    ),
                    (
                        format!("db 0x{:02X}, 0x{:02X}", port_bytes[0], port_bytes[1]),
                        format!(".byte 0x{:02X}, 0x{:02X}", port_bytes[0], port_bytes[1]),
                        format!("sin_port = {} (network byte order)", self.ipv4_port),
                    ),
                    (
                        format!("db {}, {}, {}, {}", ip_octets[0], ip_octets[1], ip_octets[2], ip_octets[3]),
                        format!(".byte {}, {}, {}, {}", ip_octets[0], ip_octets[1], ip_octets[2], ip_octets[3]),
                        format!("sin_addr = {}", ip),
                    ),
                    (
                        "db 0, 0, 0, 0, 0, 0, 0, 0".to_string(),
                        ".byte 0, 0, 0, 0, 0, 0, 0, 0".to_string(),
                        "sin_zero (padding)".to_string(),
                    ),
                ];

                let lines_ref: Vec<(&str, &str, &str)> = lines.iter()
                    .map(|(intel, att, comm)| (intel.as_str(), att.as_str(), comm.as_str()))
                    .collect();

                let assembly = format_assembly_block(&lines_ref, att_syntax, &format!("sockaddr_in (IPv4: {}, Port: {})", ip, self.ipv4_port));

                Ok(PackedResult {
                    full_bytes: bytes.clone(),
                    minimal_bytes: bytes,
                    assembly_full: assembly.clone(),
                    assembly_minimal: assembly,
                })
            }
            StructType::SockAddrIn6 => {
                let ip: Ipv6Addr = self.ipv6_addr.trim().parse()
                    .map_err(|e| format!("Invalid IPv6 address: {}", e))?;
                
                let port_bytes = self.ipv6_port.to_be_bytes();
                let flow_bytes = self.ipv6_flowinfo.to_be_bytes();
                let ip_octets = ip.octets();
                let scope_bytes = self.ipv6_scope_id.to_ne_bytes();
                
                let mut bytes = Vec::new();
                bytes.extend_from_slice(&(10u16).to_ne_bytes()); // AF_INET6 = 10
                bytes.extend_from_slice(&port_bytes);
                bytes.extend_from_slice(&flow_bytes);
                bytes.extend_from_slice(&ip_octets);
                bytes.extend_from_slice(&scope_bytes);

                let ip_octets_str = ip_octets.iter().map(|b| format!("0x{:02X}", b)).collect::<Vec<_>>().join(", ");
                let flow_bytes_str = flow_bytes.iter().map(|b| format!("0x{:02X}", b)).collect::<Vec<_>>().join(", ");
                let scope_bytes_str = scope_bytes.iter().map(|b| format!("0x{:02X}", b)).collect::<Vec<_>>().join(", ");

                let lines = vec![
                    (
                        "dw 10".to_string(),
                        ".short 10".to_string(),
                        "sin6_family = AF_INET6 (10)".to_string(),
                    ),
                    (
                        format!("db 0x{:02X}, 0x{:02X}", port_bytes[0], port_bytes[1]),
                        format!(".byte 0x{:02X}, 0x{:02X}", port_bytes[0], port_bytes[1]),
                        format!("sin6_port = {} (network byte order)", self.ipv6_port),
                    ),
                    (
                        format!("db {}", flow_bytes_str),
                        format!(".byte {}", flow_bytes_str),
                        format!("sin6_flowinfo = {}", self.ipv6_flowinfo),
                    ),
                    (
                        format!("db {}", ip_octets_str),
                        format!(".byte {}", ip_octets_str),
                        format!("sin6_addr = {}", ip),
                    ),
                    (
                        format!("db {}", scope_bytes_str),
                        format!(".byte {}", scope_bytes_str),
                        format!("sin6_scope_id = {} (host byte order)", self.ipv6_scope_id),
                    ),
                ];

                let lines_ref: Vec<(&str, &str, &str)> = lines.iter()
                    .map(|(intel, att, comm)| (intel.as_str(), att.as_str(), comm.as_str()))
                    .collect();

                let assembly = format_assembly_block(&lines_ref, att_syntax, &format!("sockaddr_in6 (IPv6: {}, Port: {})", ip, self.ipv6_port));

                Ok(PackedResult {
                    full_bytes: bytes.clone(),
                    minimal_bytes: bytes,
                    assembly_full: assembly.clone(),
                    assembly_minimal: assembly,
                })
            }
            StructType::SockAddrUn => {
                let parsed_path = parse_escaped_string(&self.unix_path)?;
                let mut path_bytes = parsed_path.clone();
                if self.unix_abstract {
                    if !path_bytes.starts_with(&[0]) {
                        path_bytes.insert(0, 0);
                    }
                } else {
                    path_bytes.push(0); // null terminator for standard paths
                }

                if path_bytes.len() > 108 {
                    return Err(format!("Unix socket path is too long ({} bytes, max 108)", path_bytes.len()));
                }

                let mut full_path_bytes = path_bytes.clone();
                let minimal_len = 2 + path_bytes.len();
                full_path_bytes.resize(108, 0); // pad path component to 108 bytes

                let mut full_bytes = Vec::new();
                full_bytes.extend_from_slice(&(1u16).to_ne_bytes()); // AF_UNIX = 1
                full_bytes.extend_from_slice(&full_path_bytes);

                let mut minimal_bytes = Vec::new();
                minimal_bytes.extend_from_slice(&(1u16).to_ne_bytes());
                minimal_bytes.extend_from_slice(&path_bytes);

                let path_desc = if self.unix_abstract {
                    format!("abstract: {:?}", String::from_utf8_lossy(&path_bytes[1..]))
                } else {
                    format!("{:?}", String::from_utf8_lossy(&path_bytes[..path_bytes.len() - 1]))
                };

                // Full assembly representation
                let full_path_str = full_path_bytes.iter().map(|b| format!("0x{:02X}", b)).collect::<Vec<_>>().join(", ");
                let lines_full = vec![
                    (
                        "dw 1".to_string(),
                        ".short 1".to_string(),
                        "sun_family = AF_UNIX (1)".to_string(),
                    ),
                    (
                        format!("db {}", full_path_str),
                        format!(".byte {}", full_path_str),
                        format!("sun_path = {} (padded to 108 bytes)", path_desc),
                    ),
                ];

                let lines_full_ref: Vec<(&str, &str, &str)> = lines_full.iter()
                    .map(|(intel, att, comm)| (intel.as_str(), att.as_str(), comm.as_str()))
                    .collect();

                let assembly_full = format_assembly_block(&lines_full_ref, att_syntax, &format!("sockaddr_un (Unix: {}) - Full 110-byte struct", self.unix_path));

                // Minimal assembly representation
                let min_path_str = path_bytes.iter().map(|b| format!("0x{:02X}", b)).collect::<Vec<_>>().join(", ");
                let lines_min = vec![
                    (
                        "dw 1".to_string(),
                        ".short 1".to_string(),
                        "sun_family = AF_UNIX (1)".to_string(),
                    ),
                    (
                        format!("db {}", min_path_str),
                        format!(".byte {}", min_path_str),
                        format!("sun_path = {}", path_desc),
                    ),
                ];

                let lines_min_ref: Vec<(&str, &str, &str)> = lines_min.iter()
                    .map(|(intel, att, comm)| (intel.as_str(), att.as_str(), comm.as_str()))
                    .collect();

                let assembly_minimal = format_assembly_block(&lines_min_ref, att_syntax, &format!("sockaddr_un (Unix: {}) - Minimal {} bytes", self.unix_path, minimal_len));

                Ok(PackedResult {
                    full_bytes,
                    minimal_bytes,
                    assembly_full,
                    assembly_minimal,
                })
            }
            StructType::SockAddr => {
                let data_bytes = parse_hex_string(&self.generic_data)?;
                if data_bytes.len() > 14 {
                    return Err(format!("Generic sockaddr data too long ({} bytes, max 14)", data_bytes.len()));
                }

                let mut full_data_bytes = data_bytes.clone();
                let minimal_len = 2 + data_bytes.len();
                full_data_bytes.resize(14, 0);

                let mut full_bytes = Vec::new();
                full_bytes.extend_from_slice(&self.generic_family.to_ne_bytes());
                full_bytes.extend_from_slice(&full_data_bytes);

                let mut minimal_bytes = Vec::new();
                minimal_bytes.extend_from_slice(&self.generic_family.to_ne_bytes());
                minimal_bytes.extend_from_slice(&data_bytes);

                // Full assembly
                let full_data_str = full_data_bytes.iter().map(|b| format!("0x{:02X}", b)).collect::<Vec<_>>().join(", ");
                let lines_full = vec![
                    (
                        format!("dw {}", self.generic_family),
                        format!(".short {}", self.generic_family),
                        format!("sa_family = {}", self.generic_family),
                    ),
                    (
                        format!("db {}", full_data_str),
                        format!(".byte {}", full_data_str),
                        format!("sa_data = (padded to 14 bytes)"),
                    ),
                ];

                let lines_full_ref: Vec<(&str, &str, &str)> = lines_full.iter()
                    .map(|(intel, att, comm)| (intel.as_str(), att.as_str(), comm.as_str()))
                    .collect();

                let assembly_full = format_assembly_block(&lines_full_ref, att_syntax, &format!("sockaddr (Generic) - Full 16-byte struct"));

                // Minimal assembly
                let min_data_str = data_bytes.iter().map(|b| format!("0x{:02X}", b)).collect::<Vec<_>>().join(", ");
                let lines_min = vec![
                    (
                        format!("dw {}", self.generic_family),
                        format!(".short {}", self.generic_family),
                        format!("sa_family = {}", self.generic_family),
                    ),
                    (
                        format!("db {}", min_data_str),
                        format!(".byte {}", min_data_str),
                        format!("sa_data ({} bytes)", data_bytes.len()),
                    ),
                ];

                let lines_min_ref: Vec<(&str, &str, &str)> = lines_min.iter()
                    .map(|(intel, att, comm)| (intel.as_str(), att.as_str(), comm.as_str()))
                    .collect();

                let assembly_minimal = format_assembly_block(&lines_min_ref, att_syntax, &format!("sockaddr (Generic) - Minimal {} bytes", minimal_len));

                Ok(PackedResult {
                    full_bytes,
                    minimal_bytes,
                    assembly_full,
                    assembly_minimal,
                })
            }
        }
    }
}

fn format_assembly_block(
    lines: &[(&str, &str, &str)],
    att_syntax: bool,
    description: &str,
) -> String {
    let mut out = String::new();
    if att_syntax {
        out.push_str(&format!("/* {} */\n", description));
        for &(_intel_dir, att_dir, comment) in lines {
            out.push_str(&format!("{:<30} /* {} */\n", att_dir, comment));
        }
    } else {
        out.push_str(&format!("; {}\n", description));
        for &(intel_dir, _att_dir, comment) in lines {
            out.push_str(&format!("{:<30} ; {}\n", intel_dir, comment));
        }
    }
    out
}

fn parse_escaped_string(s: &str) -> Result<Vec<u8>, String> {
    let mut bytes = Vec::new();
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('0') => {
                    bytes.push(0);
                }
                Some('x') => {
                    let mut hex_str = String::new();
                    if let Some(h1) = chars.next() {
                        hex_str.push(h1);
                    } else {
                        return Err("Invalid hex escape: incomplete".to_string());
                    }
                    if let Some(h2) = chars.next() {
                        hex_str.push(h2);
                    } else {
                        return Err("Invalid hex escape: incomplete".to_string());
                    }
                    let val = u8::from_str_radix(&hex_str, 16)
                        .map_err(|e| format!("Invalid hex escape: {}", e))?;
                    bytes.push(val);
                }
                Some('n') => bytes.push(b'\n'),
                Some('r') => bytes.push(b'\r'),
                Some('t') => bytes.push(b'\t'),
                Some('\\') => bytes.push(b'\\'),
                Some(other) => {
                    bytes.push(b'\\');
                    let mut buf = [0; 4];
                    for &b in other.encode_utf8(&mut buf).as_bytes() {
                        bytes.push(b);
                    }
                }
                None => {
                    bytes.push(b'\\');
                }
            }
        } else {
            let mut buf = [0; 4];
            for &b in c.encode_utf8(&mut buf).as_bytes() {
                bytes.push(b);
            }
        }
    }
    Ok(bytes)
}

fn parse_hex_string(s: &str) -> Result<Vec<u8>, String> {
    let mut bytes = Vec::new();
    let cleaned: String = s.chars().filter(|c| !c.is_whitespace() && *c != ',').collect();
    if cleaned.len() % 2 != 0 {
        return Err("Hex string must have an even number of characters".to_string());
    }
    for chunk in cleaned.as_bytes().chunks(2) {
        let s = std::str::from_utf8(chunk).map_err(|e| e.to_string())?;
        let val = u8::from_str_radix(s, 16).map_err(|e| format!("Invalid hex byte '{}': {}", s, e))?;
        bytes.push(val);
    }
    Ok(bytes)
}

pub fn render_struct_packer_panel(app: &mut ShellcideApp, ui: &mut egui::Ui) {
    ui.heading("libc Struct Packer");
    ui.add_space(4.0);

    // Structure selector
    ui.horizontal(|ui| {
        ui.selectable_value(&mut app.struct_packer.selected_struct, StructType::SockAddrIn, "IPv4");
        ui.selectable_value(&mut app.struct_packer.selected_struct, StructType::SockAddrIn6, "IPv6");
        ui.selectable_value(&mut app.struct_packer.selected_struct, StructType::SockAddrUn, "Unix");
        ui.selectable_value(&mut app.struct_packer.selected_struct, StructType::SockAddr, "Generic");
    });
    ui.separator();

    // Inputs form
    egui::Grid::new("struct_packer_inputs_grid")
        .num_columns(2)
        .spacing([12.0, 8.0])
        .show(ui, |ui| {
            match app.struct_packer.selected_struct {
                StructType::SockAddrIn => {
                    ui.label("IP Address:");
                    ui.add(egui::TextEdit::singleline(&mut app.struct_packer.ipv4_addr).desired_width(180.0));
                    ui.end_row();
                    
                    ui.label("Port:");
                    ui.add(egui::DragValue::new(&mut app.struct_packer.ipv4_port).range(1..=65535));
                    ui.end_row();
                }
                StructType::SockAddrIn6 => {
                    ui.label("IPv6 Address:");
                    ui.add(egui::TextEdit::singleline(&mut app.struct_packer.ipv6_addr).desired_width(180.0));
                    ui.end_row();
                    
                    ui.label("Port:");
                    ui.add(egui::DragValue::new(&mut app.struct_packer.ipv6_port).range(1..=65535));
                    ui.end_row();

                    ui.label("Flow Info:");
                    ui.add(egui::DragValue::new(&mut app.struct_packer.ipv6_flowinfo));
                    ui.end_row();

                    ui.label("Scope ID:");
                    ui.add(egui::DragValue::new(&mut app.struct_packer.ipv6_scope_id));
                    ui.end_row();
                }
                StructType::SockAddrUn => {
                    ui.label("Socket Path:");
                    ui.add(egui::TextEdit::singleline(&mut app.struct_packer.unix_path).desired_width(180.0));
                    ui.end_row();
                    
                    ui.label("Abstract Namespace:");
                    ui.checkbox(&mut app.struct_packer.unix_abstract, "Prefix with \\0");
                    ui.end_row();
                }
                StructType::SockAddr => {
                    ui.label("Address Family:");
                    ui.add(egui::DragValue::new(&mut app.struct_packer.generic_family));
                    ui.end_row();

                    ui.label("Data (Hex):");
                    ui.add(egui::TextEdit::singleline(&mut app.struct_packer.generic_data).desired_width(180.0));
                    ui.end_row();
                }
            }
        });

    ui.separator();

    // Packing and Rendering Output
    match app.struct_packer.pack(app.att_syntax) {
        Ok(result) => {
            egui::ScrollArea::vertical().id_salt("packer_output_scroll").show(ui, |ui| {
                let show_minimal = result.full_bytes != result.minimal_bytes;

                if show_minimal {
                    ui.collapsing(format!("Minimal Representation ({} bytes)", result.minimal_bytes.len()), |ui| {
                        render_packed_output_details(ui, app, &result.minimal_bytes, &result.assembly_minimal);
                    });
                    ui.collapsing(format!("Full Structure ({} bytes)", result.full_bytes.len()), |ui| {
                        render_packed_output_details(ui, app, &result.full_bytes, &result.assembly_full);
                    });
                } else {
                    render_packed_output_details(ui, app, &result.full_bytes, &result.assembly_full);
                }
            });
        }
        Err(err) => {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("⚠️").color(Color32::from_rgb(255, 118, 117)));
                ui.colored_label(Color32::from_rgb(255, 118, 117), err);
            });
        }
    }
}

fn format_little_endian_hex(bytes: &[u8]) -> String {
    let mut words = Vec::new();
    for chunk in bytes.chunks(8) {
        let mut padded = [0u8; 8];
        padded[..chunk.len()].copy_from_slice(chunk);
        let val = u64::from_le_bytes(padded);
        words.push(format!("0x{:016X}", val));
    }
    words.join(", ")
}

fn format_stack_push_assembly(bytes: &[u8], att_syntax: bool) -> String {
    let mut chunks = Vec::new();
    for chunk in bytes.chunks(8) {
        let mut padded = [0u8; 8];
        padded[..chunk.len()].copy_from_slice(chunk);
        let val = u64::from_le_bytes(padded);
        chunks.push(val);
    }

    let mut out = String::new();
    if att_syntax {
        out.push_str("/* Stack push sequence (reverse order) */\n");
        for (i, &val) in chunks.iter().enumerate().rev() {
            let start_byte = i * 8;
            let end_byte = (i * 8 + 7).min(bytes.len() - 1);
            if val <= 0x7FFFFFFF {
                out.push_str(&format!("pushq $0x{:08X}   /* bytes {}..{} */\n", val, start_byte, end_byte));
            } else {
                out.push_str(&format!("movabs $0x{:016X}, %rax\n", val));
                out.push_str(&format!("pushq %rax         /* bytes {}..{} */\n", start_byte, end_byte));
            }
        }
    } else {
        out.push_str("; Stack push sequence (reverse order)\n");
        for (i, &val) in chunks.iter().enumerate().rev() {
            let start_byte = i * 8;
            let end_byte = (i * 8 + 7).min(bytes.len() - 1);
            if val <= 0x7FFFFFFF {
                out.push_str(&format!("push 0x{:08X}      ; bytes {}..{}\n", val, start_byte, end_byte));
            } else {
                out.push_str(&format!("mov rax, 0x{:016X}\n", val));
                out.push_str(&format!("push rax           ; bytes {}..{}\n", start_byte, end_byte));
            }
        }
    }
    out
}

fn render_packed_output_details(ui: &mut egui::Ui, app: &mut ShellcideApp, bytes: &[u8], assembly: &str) {
    // 1. Action buttons
    ui.horizontal(|ui| {
        if ui.button("➕ Insert Assembly at Cursor").clicked() {
            app.insert_into_editor(ui.ctx(), assembly);
        }
        if ui.button("📋 Copy Assembly").clicked() {
            ui.ctx().copy_text(assembly.to_string());
        }
    });

    ui.add_space(4.0);

    // 2. Assembly View
    ui.label("Assembly directives:");
    ui.add(
        egui::TextEdit::multiline(&mut assembly.to_string())
            .font(egui::FontId::monospace(12.0))
            .desired_rows(4)
            .desired_width(f32::INFINITY)
            .interactive(false)
    );

    ui.add_space(4.0);

    // 3. Hex & Python/C representations
    let hex_str = bytes.iter().map(|b| format!("{:02x}", b)).collect::<String>();
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("Hex:").weak());
        ui.monospace(&hex_str);
        if ui.button("📋").on_hover_text("Copy Hex").clicked() {
            ui.ctx().copy_text(hex_str);
        }
    });

    let le_hex_str = format_little_endian_hex(bytes);
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("Little-Endian (64-bit):").weak());
        ui.monospace(&le_hex_str);
        if ui.button("📋").on_hover_text("Copy Little-Endian hex").clicked() {
            ui.ctx().copy_text(le_hex_str);
        }
    });

    let mut py_str = "b\"".to_string();
    for &b in bytes {
        py_str.push_str(&format!("\\x{:02x}", b));
    }
    py_str.push('"');

    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("Python/C:").weak());
        ui.monospace(&py_str);
        if ui.button("📋").on_hover_text("Copy Python/C buffer").clicked() {
            ui.ctx().copy_text(py_str);
        }
    });

    ui.add_space(4.0);

    // 5. Stack Push Assembly View
    let push_assembly = format_stack_push_assembly(bytes, app.att_syntax);
    ui.collapsing("Stack Push Assembly Sequence", |ui| {
        ui.horizontal(|ui| {
            if ui.button("➕ Insert Push at Cursor").clicked() {
                app.insert_into_editor(ui.ctx(), &push_assembly);
            }
            if ui.button("📋 Copy Push Sequence").clicked() {
                ui.ctx().copy_text(push_assembly.to_string());
            }
        });
        ui.add_space(2.0);
        ui.add(
            egui::TextEdit::multiline(&mut push_assembly.to_string())
                .font(egui::FontId::monospace(12.0))
                .desired_rows(4)
                .desired_width(f32::INFINITY)
                .interactive(false)
        );
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_escaped_string() {
        assert_eq!(parse_escaped_string("/tmp/sock").unwrap(), b"/tmp/sock");
        assert_eq!(parse_escaped_string("\\0test").unwrap(), b"\0test");
        assert_eq!(parse_escaped_string("a\\nb").unwrap(), b"a\nb");
        assert_eq!(parse_escaped_string("\\x41\\x42").unwrap(), b"AB");
    }

    #[test]
    fn test_parse_hex_string() {
        assert_eq!(parse_hex_string("00 11 22").unwrap(), vec![0x00, 0x11, 0x22]);
        assert_eq!(parse_hex_string("aa, bb, cc").unwrap(), vec![0xaa, 0xbb, 0xcc]);
        assert!(parse_hex_string("a").is_err());
    }

    #[test]
    fn test_pack_sockaddr_in() {
        let mut state = StructPackerState::default();
        state.selected_struct = StructType::SockAddrIn;
        state.ipv4_addr = "127.0.0.1".to_string();
        state.ipv4_port = 4444;

        let res = state.pack(false).unwrap();
        assert_eq!(res.full_bytes.len(), 16);
        assert_eq!(res.full_bytes[0..2], (2u16).to_ne_bytes());
        assert_eq!(res.full_bytes[2..4], 4444u16.to_be_bytes());
        assert_eq!(res.full_bytes[4..8], [127, 0, 0, 1]);
        assert_eq!(res.full_bytes[8..16], [0; 8]);
    }

    #[test]
    fn test_pack_sockaddr_in6() {
        let mut state = StructPackerState::default();
        state.selected_struct = StructType::SockAddrIn6;
        state.ipv6_addr = "::1".to_string();
        state.ipv6_port = 8080;
        state.ipv6_flowinfo = 0;
        state.ipv6_scope_id = 0;

        let res = state.pack(false).unwrap();
        assert_eq!(res.full_bytes.len(), 28);
        assert_eq!(res.full_bytes[0..2], (10u16).to_ne_bytes());
        assert_eq!(res.full_bytes[2..4], 8080u16.to_be_bytes());
        assert_eq!(res.full_bytes[4..8], [0; 4]); // flowinfo
        
        let mut expected_ip = [0u8; 16];
        expected_ip[15] = 1;
        assert_eq!(res.full_bytes[8..24], expected_ip);
        assert_eq!(res.full_bytes[24..28], [0; 4]); // scope id
    }

    #[test]
    fn test_pack_sockaddr_un() {
        let mut state = StructPackerState::default();
        state.selected_struct = StructType::SockAddrUn;
        state.unix_path = "abc".to_string();
        state.unix_abstract = true;

        let res = state.pack(false).unwrap();
        assert_eq!(res.full_bytes.len(), 110);
        assert_eq!(res.full_bytes[0..2], (1u16).to_ne_bytes());
        assert_eq!(res.full_bytes[2..6], [0, b'a', b'b', b'c']); // \0abc
        assert_eq!(res.minimal_bytes.len(), 6); // family (2) + \0abc (4)
    }

    #[test]
    fn test_format_little_endian_hex() {
        let bytes = vec![0x02, 0x00, 0x11, 0x5c, 0x7f, 0x00, 0x00, 0x01];
        assert_eq!(format_little_endian_hex(&bytes), "0x0100007F5C110002");

        let bytes_short = vec![0x02, 0x00];
        assert_eq!(format_little_endian_hex(&bytes_short), "0x0000000000000002");
    }

    #[test]
    fn test_format_stack_push_assembly() {
        let bytes = vec![0x02, 0x00, 0x11, 0x5c, 0x7f, 0x00, 0x00, 0x01];
        let assembly = format_stack_push_assembly(&bytes, false);
        assert!(assembly.contains("mov rax, 0x0100007F5C110002"));
        assert!(assembly.contains("push rax"));
    }
}

