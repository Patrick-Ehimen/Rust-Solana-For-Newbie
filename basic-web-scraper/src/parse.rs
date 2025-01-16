use scraper::{Html, Selector};

pub fn parse_html(html: &str) -> Vec<String> {
    let document = Html::parse_document(html);
    let selector = Selector::parse("h1, h2, h3, p, a").unwrap();
    document
        .select(&selector)
        .map(|element| element.text().collect::<Vec<_>>().join(" "))
        .collect()
}
