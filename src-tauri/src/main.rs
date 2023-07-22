// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

/// Access a string property, default to empty string if missing
macro_rules! get_str {
  ($obj:expr, $key:expr) => {
    $obj.get($key)
         .map_or("", |v| v.as_str().unwrap())
  };
}

#[tauri::command]
fn submit_claim(claim: serde_json::Value) {

  let name = get_str!(claim, "customer_name");
  let email = get_str!(claim, "customer_email");
  println!("{claim:?}");
  println!("{name} : {email}");

}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![submit_claim])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
