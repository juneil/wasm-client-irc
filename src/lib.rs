use wasm_bindgen::prelude::*;
pub mod element;
mod input;

#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (log(&format!("[wasm] {}", &format_args!($($t)*).to_string()).to_string()))
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    console_log!("version {}", VERSION);

    // let elem = element::Element::query_selector("#form_input")
    //     .unwrap()
    //     .add_event_listener("keypress", move |e: web_sys::KeyboardEvent| {
    //          console_log!("{:?}", e.key_code());
    //     });

    let i = input::InputForm::new();

    Ok(())
}
