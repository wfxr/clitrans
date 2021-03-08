use scraper::{ElementRef, Html, Selector};

pub(crate) fn get_element<'a>(doc: &'a Html, selector: &str) -> Option<ElementRef<'a>> {
    let selector = Selector::parse(selector).unwrap();
    doc.select(&selector).next()
}

pub(crate) fn get_text(element: ElementRef, selector: &str) -> Vec<String> {
    element
        .select(&Selector::parse(selector).unwrap())
        .map(|e| e.text().collect())
        .collect()
}
