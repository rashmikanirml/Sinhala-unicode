
use std::collections::HashMap;
use src_tauri::lib::transliterate::{transliterate, ExceptionEntry};

#[test]
fn test_transliterate_basic() {
	// Hardcoded mapping for test
	let mut mapping = HashMap::new();
	mapping.insert("ka".to_string(), "ක".to_string());
	mapping.insert("vi".to_string(), "වි".to_string());
	mapping.insert("a".to_string(), "අ".to_string());
	mapping.insert("m".to_string(), "ම".to_string());

	let mut exceptions = HashMap::new();
	exceptions.insert(
		"amma".to_string(),
		ExceptionEntry {
			sinhala: Some("අම්මා".to_string()),
			tamil: Some("அம்மா".to_string()),
		},
	);

	// Test exception
	assert_eq!(transliterate("amma", &mapping, &exceptions, "sinhala"), "අම්මා");
	assert_eq!(transliterate("amma", &mapping, &exceptions, "tamil"), "அம்மா");

	// Test rule-based
	assert_eq!(transliterate("kavi", &mapping, &exceptions, "sinhala"), "කවි");
	assert_eq!(transliterate("a", &mapping, &exceptions, "sinhala"), "අ");
	assert_eq!(transliterate("m", &mapping, &exceptions, "sinhala"), "ම");
}
