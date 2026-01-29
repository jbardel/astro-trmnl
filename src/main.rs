use std::collections::HashMap;

use scraper::{Html, Node, Selector};

const URL: &str = "https://www.astroo.com/horoscope.php";
const PARENT_SELECTOR: &str = ".hqte";
const SIGN_SELECTOR: &str = ".hqti a";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let body = reqwest::blocking::get(URL)?.text()?;
    let document = Html::parse_document(&body);

    let parent_selector = Selector::parse(PARENT_SELECTOR)?;
    let sign_selector = Selector::parse(SIGN_SELECTOR)?;

    let mut map: HashMap<String, String> = HashMap::new();
    for parent in document.select(&parent_selector) {
        let sign = parent
            .select(&sign_selector)
            .next()
            .map(|e| e.text().collect::<String>())
            .unwrap_or_else(|| "Inconnu".to_string());

        let description = parent
            .children()
            .filter_map(|child| match child.value() {
                Node::Text(t) => Some(t.text.trim()),
                _ => None,
            })
            .find(|t| !t.is_empty())
            .unwrap_or("")
            .to_string();

        map.insert(sign, description);
    }

    println!("{:?}", map);

    Ok(())
}
