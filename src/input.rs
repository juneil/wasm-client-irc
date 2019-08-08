use super::element;

#[macro_use]
use super::console_log;

#[derive(Debug, Clone)]
pub struct InputForm {
    element: Option<element::Element>
}

impl InputForm {
    pub fn new() -> InputForm {
        let mut input = InputForm {
            element: element::Element::query_selector("#input_form")
        };
        input.init_send();
        input
    }

    fn init_send(&mut self) {
        if let Some(mut element) = self.element.clone() {
            let element_cloned = element.clone();
            element.add_event_listener("keypress", move |event: web_sys::KeyboardEvent| {
                if let Some(input) = element_cloned.clone().inner {
                    if event.key_code() == 13 {
                        console_log!("/////////////");
                        input.set_attribute("value", "+").ok();
                    }
                }
                // let mut input: web_sys::HtmlInputElement = element.inner.unwrap().into();
            });
        }
    }
}
