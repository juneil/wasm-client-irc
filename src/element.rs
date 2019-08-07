
pub struct Element {
    pub inner: Option<web_sys::Element>
}

impl From<web_sys::Element> for Element {
    fn from(element: web_sys::Element) -> Element {
        Element {
            inner: Some(element)
        }
    }
}

impl From<Element> for Option<web_sys::Node> {
    fn from(element: Element) -> Option<web_sys::Node> {
        if let Some(inner) = element.inner {
            Some(inner.into())
        } else {
            None
        }
    }
}

impl Element {

    pub fn create(tag: &str) -> Option<Element> {
        web_sys::window()
            .and_then(|window| window.document())
            .and_then(|document| document.create_element(tag).ok())
            .map(|element| Element::from(element))
    }

    pub fn get_body() -> Option<web_sys::HtmlElement> {
        web_sys::window()
            .and_then(|window| window.document())
            .and_then(|document| document.body())
    }

}
