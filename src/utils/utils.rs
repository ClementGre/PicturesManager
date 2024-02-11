use serde::{de::DeserializeOwned, Serialize};
use tauri_sys::tauri::invoke;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use web_sys::HtmlElement;
use yew::NodeRef;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"], js_name = "invoke")]
    fn invoke_tauri(cmd: &str, args: JsValue) -> JsValue;
}

/// Invoke a tauri command without arguments and without return value.
#[allow(unused_must_use)]
pub fn cmd(cmd: &str) {
    invoke_tauri(&cmd, JsValue::default());
}
/// Invoke a tauri command with arguments and without return value.
pub fn cmd_arg<A: Serialize>(cmd: &str, arg: &A) {
    invoke_tauri(
        &cmd,
        serde_wasm_bindgen::to_value(arg).expect(format!("Unable to serialize argument for command {}", cmd).as_str()),
    );
}
/// Invoke a tauri command without arguments and with return value. Async function
pub async fn cmd_async_get<R: DeserializeOwned>(cmd: &str) -> R {
    invoke(cmd, &()).await.expect(format!("Unable to invoke async command {}", cmd).as_str())
}
/// Invoke a tauri command with arguments and with return value. Async function
pub async fn cmd_async<A: Serialize, R: DeserializeOwned>(cmd: &str, arg: &A) -> R {
    invoke(cmd, arg).await.expect(format!("Unable to invoke async command {}", cmd).as_str())
}

pub fn get_non_null_ref(ref_1: NodeRef, ref_2: NodeRef) -> Option<HtmlElement> {
    if let Some(element) = ref_1.cast::<HtmlElement>() {
        return Some(element);
    } else if let Some(element) = ref_2.cast::<HtmlElement>() {
        return Some(element);
    }
    return None;
}
