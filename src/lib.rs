use wasm_bindgen::prelude::*;

mod console;
pub mod element;
mod input;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    console_log!("version {}", VERSION);

    let _: Option<()> = element::Element::get_body()
        .and_then(|body| body.style().set_property("background-color", "black").ok());

    Ok(())
}
