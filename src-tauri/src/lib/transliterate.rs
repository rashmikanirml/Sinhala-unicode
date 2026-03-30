use std::collections::HashMap;
use std::fs;
use std::path::Path;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ExceptionEntry {
	pub sinhala: Option<String>,
	pub tamil: Option<String>,
}

pub fn load_mapping<P: AsRef<Path>>(path: P) -> HashMap<String, String> {
	let data = fs::read_to_string(path).expect("Failed to read mapping file");
	serde_json::from_str(&data).unwrap_or_default()
}

pub fn load_exceptions<P: AsRef<Path>>(path: P) -> HashMap<String, ExceptionEntry> {
	let data = fs::read_to_string(path).expect("Failed to read exceptions file");
	serde_json::from_str(&data).unwrap_or_default()
}

/// Transliterate a Singlish word to Sinhala or Tamil using mapping and exceptions.
pub fn transliterate(
	input: &str,
	mapping: &HashMap<String, String>,
	exceptions: &HashMap<String, ExceptionEntry>,
	target: &str, // "sinhala" or "tamil"
) -> String {
	// Check exceptions first
	if let Some(entry) = exceptions.get(input) {
		if target == "sinhala" {
			if let Some(val) = &entry.sinhala {
				return val.clone();
			}
		} else if target == "tamil" {
			if let Some(val) = &entry.tamil {
				return val.clone();
			}
		}
	}
	// Simple rule-based: longest match first
	let mut output = String::new();
	let mut i = 0;
	let chars: Vec<char> = input.chars().collect();
	while i < chars.len() {
		let mut found = false;
		// Try longest possible match (up to 4 chars)
		for len in (1..=4).rev() {
			if i + len <= chars.len() {
				let candidate: String = chars[i..i+len].iter().collect();
				if let Some(val) = mapping.get(&candidate) {
					output.push_str(val);
					i += len;
					found = true;
					break;
				}
			}
		}
		if !found {
			// Fallback: copy as-is
			output.push(chars[i]);
			i += 1;
		}
	}
	output
}
