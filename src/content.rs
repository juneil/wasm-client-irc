use super::element;

#[derive(Debug, Clone)]
pub struct Content {
    element: Option<element::Element>
}

impl Content {
    pub fn new() -> Content {
        Content {
            element: element::Element::query_selector("#content")
        }
    }

    pub fn insert_message(&self, message: &str, source: Option<&str>) {
        if let Some(element) = self.element.clone().and_then(|e| e.inner) {
            element.insert_adjacent_html("beforeend",
                format!("<p><span>{}</span>{}</p>", source.map(|x| format!("{} :", x)).unwrap_or(String::from("")), message)
                    .as_ref()
            ).ok();
        }
    }


}