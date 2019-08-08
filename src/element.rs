use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::EventTarget;

#[derive(Debug, Clone)]
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
        Element::get_document()
            .and_then(|document| document.create_element(tag).ok())
            .map(|element| Element::from(element))
    }

    pub fn get_document() -> Option<web_sys::Document> {
        web_sys::window()
            .and_then(|window| window.document())
    }

    pub fn get_body() -> Option<web_sys::HtmlElement> {
        Element::get_document()
            .and_then(|document| document.body())
    }

    pub fn query_selector(qs: &str) -> Option<Element> {
        Element::get_document()
            .and_then(|document| document.query_selector(qs)
                .ok()
                .unwrap_or(None)
            )
            .map(|element| Element::from(element))
    }

    pub fn add_event_listener<T, U>(&mut self, event_name: &str, handler: T)
    where
        T: 'static + FnMut(U),
        U: 'static + wasm_bindgen::convert::FromWasmAbi
    {
        let cb = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);
        if let Some(el) = self.inner.take() {
            let el_et: EventTarget = el.into();
            el_et
                .add_event_listener_with_callback(event_name, cb.as_ref().unchecked_ref())
                .unwrap();
            cb.forget();
            if let Ok(el) = el_et.dyn_into::<web_sys::Element>() {
                self.inner = Some(el);
            }
        }
    }
}
