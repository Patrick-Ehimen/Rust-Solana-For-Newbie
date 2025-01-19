pub struct Url {
    pub id: String,
    pub original_url: String,
    pub short_url: String,
}

impl Url {
    pub fn new(id: String, original_url: String, short_url: String) -> Self {
        Url {
            id,
            original_url,
            short_url,
        }
    }
}