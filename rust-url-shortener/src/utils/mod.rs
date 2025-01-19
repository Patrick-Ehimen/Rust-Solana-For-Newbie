use rand::Rng;

pub fn generate_short_url() -> String {
    let charset: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"
        .chars()
        .collect();
    let mut rng = rand::thread_rng();
    (0..6)
        .map(|_| charset[rng.gen_range(0..charset.len())])
        .collect()
}

pub fn is_valid_url(url: &str) -> bool {
    url::Url::parse(url).is_ok()
}
