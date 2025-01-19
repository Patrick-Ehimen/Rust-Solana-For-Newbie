use crate::utils::generate_short_url;
use lazy_static::lazy_static;
use log::info;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    static ref URL_MAP: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

pub fn create_short_url(original_url: &str) -> String {
    let short_url = generate_short_url();
    URL_MAP
        .lock()
        .unwrap()
        .insert(short_url.clone(), original_url.to_string());
    info!("Stored URL: {} -> {}", short_url, original_url);
    short_url
}

pub fn redirect_to_original_url(short_url: &str) -> Option<String> {
    let original_url = URL_MAP.lock().unwrap().get(short_url).cloned();
    info!("Retrieved URL: {} -> {:?}", short_url, original_url);
    original_url
}
