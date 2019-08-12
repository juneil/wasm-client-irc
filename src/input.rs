use wasm_bindgen::JsCast;
use super::element;
use super::log;

#[derive(Debug, Clone)]
pub struct Input {
    element: Option<element::Element>
}

impl Input {
    pub fn new() -> Input {
        let mut input = Input {
            element: element::Element::query_selector("#form_input")
        };
        input.init_send();
        input
    }

    // Initialise the input element
    // Add the focus and listen keypress
    // Clear the input when pressing the ENTER key
    fn init_send(&mut self) {
        if let Some(element) = self.element.clone() {
            let mut element_cloned = element.clone();
            if let Some(input) = element.inner.and_then(|elem| elem.dyn_into::<web_sys::HtmlInputElement>().ok()) {
                input.focus().ok();
                element_cloned.add_event_listener("keypress", move |event: web_sys::KeyboardEvent| {
                    if event.key_code() == 13 {
                        input.set_value("");
                    }
                });
            } else {
                log!("Input element is not the type input");
            }
        } else {
            log!("Input element not found in the DOM: #form_input");
        }
    }
}
