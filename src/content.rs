use super::element;
use super::irc;

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

    pub fn insert_message(&self, message: &str, source: Option<&str>, style: Option<irc::MessageStyle>) {
        if let Some(element) = self.element.clone().and_then(|e| e.inner) {
            element.insert_adjacent_html("beforeend",
                format!(
                    "{}<span>{}</span>{}</p>",
                    self::Content::convert_style_to_html(style),
                    source.map(|x| format!("{} :", x))
                        .unwrap_or(String::from("")),
                    message
                ).as_ref()
            ).ok();
        }
    }

    fn convert_style_to_html(style: Option<irc::MessageStyle>) -> String {
        match style {
            Some(irc::MessageStyle::Normal) => String::from("<p class=\"normal\">"),
            Some(irc::MessageStyle::Error) => String::from("<p class=\"error\">"),
            _ => String::from("<p>")
        }
    }
}