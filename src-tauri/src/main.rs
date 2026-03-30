
use tauri::command;
use std::collections::HashMap;
use src_tauri::lib::transliterate::{transliterate, load_mapping, load_exceptions};

#[command]
fn transliterate_text(input: String, target: String) -> String {
    // Load mapping and exceptions (for demo, reload every call; optimize later)
    let mapping_path = if target == "sinhala" {
        "src-tauri/resources/mappings/singlish_sinhala.json"
    } else {
        "src-tauri/resources/mappings/singlish_tamil.json"
    };
    let exceptions_path = "src-tauri/resources/lexicons/exceptions.json";
    let mapping: HashMap<String, String> = load_mapping(mapping_path);
    let exceptions = load_exceptions(exceptions_path);
    transliterate(&input, &mapping, &exceptions, &target)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![transliterate_text])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
