use wasm_bindgen::prelude::*;
pub mod element;
mod input;
mod content;

#[macro_export]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_2(&format!("[wasm]").into(), &format!($( $t )* ).into());
    }
}

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    log!("version {}", VERSION);

    input::InputForm::new();
    let c = content::Content::new();
    c.insert_message("YOloOo oOOO ddx dsddddd eOOO ddx dsddddd eOOO ddx dsddddd eOO ddx x dsddddd e", None);
    c.insert_message("YOloOo oOOO ddx dsddddd eOOO ddx dsddddd eOOO d", Some("Juneil"));
    c.insert_message("YOloOo oOOO ddx dsddddd eOOO ddx dsddddd eOOO d", Some("Juneil"));
    c.insert_message("YOloOo oOOO ddx dsddddd eOOO ddx dsddddd eOOO ddx dsddddd eOO ddx x dsddddd e", None);

    Ok(())
}
