use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Default, PartialEq)]
pub enum Cover {
    #[default]
    None,
    Solid {
        color: String,
    },
    Gradient {
        color1: String,
        color2: String,
    },
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct DocMetadata {
    pub title: String,
    pub icon: String,
    pub cover: Cover,
    pub cover_offset_y: f64,
}

pub struct MetadataStore;

impl MetadataStore {
    pub fn storage_file() -> PathBuf {
        #[cfg(target_os = "windows")]
        {
            if let Ok(app_data) = std::env::var("APPDATA") {
                let dir = PathBuf::from(app_data).join("NotionPad");
                let _ = fs::create_dir_all(&dir);
                return dir.join("page_metadata.tsv");
            }
        }

        #[cfg(not(target_os = "windows"))]
        {
            if let Ok(home) = std::env::var("HOME") {
                let dir = PathBuf::from(home).join(".config").join("notionpad");
                let _ = fs::create_dir_all(&dir);
                return dir.join("page_metadata.tsv");
            }
        }

        PathBuf::from("page_metadata.tsv")
    }

    pub fn load_all() -> HashMap<String, DocMetadata> {
        let file_path = Self::storage_file();
        let mut map = HashMap::new();

        let Ok(file) = File::open(file_path) else {
            return map;
        };

        let reader = BufReader::new(file);
        for line in reader.lines().map_while(Result::ok) {
            let parts: Vec<&str> = line.split('\t').collect();
            if parts.len() < 7 {
                continue;
            }

            let path = parts[0].to_string();
            let title = percent_decode(parts[1]);
            let icon = percent_decode(parts[2]);
            let cover_type = parts[3];
            let val1 = percent_decode(parts[4]);
            let val2 = percent_decode(parts[5]);
            let offset_y: f64 = parts[6].parse().unwrap_or(0.0);

            let cover = match cover_type {
                "solid" => Cover::Solid { color: val1 },
                "gradient" => Cover::Gradient {
                    color1: val1,
                    color2: val2,
                },
                _ => Cover::None,
            };

            map.insert(
                path,
                DocMetadata {
                    title,
                    icon,
                    cover,
                    cover_offset_y: offset_y,
                },
            );
        }

        map
    }

    pub fn get_for_path(path: &Path) -> Option<DocMetadata> {
        let key = path.to_string_lossy().to_string();
        let all = Self::load_all();
        all.get(&key).cloned()
    }

    pub fn save_for_path(path: &Path, meta: &DocMetadata) {
        let key = path.to_string_lossy().to_string();
        let mut all = Self::load_all();
        all.insert(key, meta.clone());

        let storage_path = Self::storage_file();
        let Ok(mut file) = File::create(storage_path) else {
            return;
        };

        for (item_path, item_meta) in all {
            let (cover_type, val1, val2) = match &item_meta.cover {
                Cover::None => ("none", "".to_string(), "".to_string()),
                Cover::Solid { color } => ("solid", color.clone(), "".to_string()),
                Cover::Gradient { color1, color2 } => ("gradient", color1.clone(), color2.clone()),
            };

            let line = format!(
                "{}\t{}\t{}\t{}\t{}\t{}\t{:.1}\n",
                item_path,
                percent_encode(&item_meta.title),
                percent_encode(&item_meta.icon),
                cover_type,
                percent_encode(&val1),
                percent_encode(&val2),
                item_meta.cover_offset_y
            );
            let _ = file.write_all(line.as_bytes());
        }
    }
}

pub fn percent_encode(input: &str) -> String {
    let mut encoded = String::new();
    for byte in input.as_bytes() {
        match byte {
            b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                encoded.push(*byte as char);
            }
            _ => {
                encoded.push_str(&format!("%{:02X}", byte));
            }
        }
    }
    encoded
}

pub fn percent_decode(input: &str) -> String {
    let mut bytes = Vec::new();
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '%' && i + 2 < chars.len() {
            let hex_str: String = chars[i + 1..=i + 2].iter().collect();
            if let Ok(b) = u8::from_str_radix(&hex_str, 16) {
                bytes.push(b);
                i += 3;
                continue;
            }
        }
        let mut buf = [0u8; 4];
        let encoded_char = chars[i].encode_utf8(&mut buf);
        bytes.extend_from_slice(encoded_char.as_bytes());
        i += 1;
    }
    String::from_utf8_lossy(&bytes).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn codifica_e_decodifica_percent_encoding_com_emojis_e_acentos() {
        let original = "Documento de Teste 🦀 - Olá!";
        let encoded = percent_encode(original);
        let decoded = percent_decode(&encoded);
        assert_eq!(decoded, original);
    }

    #[test]
    fn decodifica_valores_reais_do_notionpad_csharp() {
        let raw_icon = "%F0%9F%98%84"; // 😄
        assert_eq!(percent_decode(raw_icon), "😄");

        let raw_color = "%23FF416C"; // #FF416C
        assert_eq!(percent_decode(raw_color), "#FF416C");
    }
}
