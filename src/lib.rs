use wasm_bindgen::prelude::*;
pub mod element;
mod input;
mod content;
mod irc;

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

    init_ws(c);

    Ok(())
}

fn init_ws(content: content::Content) {
    if let Ok(ws) = web_sys::WebSocket::new("ws://localhost:8081") {
        let mut queue: Vec<String> = vec!();
        element::ws_add_event_listener(&ws, "open", |_: web_sys::Event| log!("WS open"));
        element::ws_add_event_listener(&ws, "close", |_: web_sys::Event| log!("WS closed"));
        element::ws_add_event_listener(&ws, "message", move |event: web_sys::MessageEvent| {
            if let Some(data) = event.data().as_string() {
                queue.push(data);
                // content.insert_message(data.as_ref(), None);
            }
            let (list, remaining) = merge_queue(queue.clone());
            queue = vec!(remaining);
            for item in list {
                content.insert_message(item.as_ref(), None);
                let cmd = irc::Command::new(item);
                log!("{:?}", cmd);
            }
        });
    };
}

fn merge_queue(queue: Vec<String>) -> (Vec<String>, String) {
    let joined = queue.join("");
    let mut splitted: Vec<String> = joined.split("\r\n").map(|s| String::from(s)).collect();
    let remaining = splitted.pop();
    (splitted, remaining.unwrap_or(String::from("")))
}