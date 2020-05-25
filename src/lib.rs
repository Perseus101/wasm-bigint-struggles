use js_sys::{Map, Function, Object, Reflect, WebAssembly};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::{spawn_local, JsFuture};

mod utils;

// lifted from the `console_log` example
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(a: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

const WASM: &[u8] = include_bytes!("native_add.wasm");

fn native_add(a: u64, b: u64) -> u64 {
    a + b
}

async fn run_async() -> Result<(), JsValue> {
    console_log!("instantiating a new wasm module directly");
    console_log!("creating imports");

    let native_add_closure = Closure::wrap(Box::new(native_add) as Box<dyn Fn(u64, u64) -> u64>);
    let native_add: &Function = native_add_closure.as_ref().unchecked_ref();
    let native_add_name = "native_add".into();

    let env = Map::new();

    env.set(&native_add_name, native_add.as_ref());

    let imports = Map::new();

    imports.set(&JsValue::from("env"), &Object::new());

    console_log!("instantiating module");
    let a = JsFuture::from(WebAssembly::instantiate_buffer(WASM, &imports.into())).await?;
    let b: WebAssembly::Instance = Reflect::get(&a, &"instance".into())?.dyn_into()?;

    let c = b.exports();

    let add = Reflect::get(c.as_ref(), &"add".into())?
        .dyn_into::<Function>()
        .expect("add export wasn't a function");

    let three = add.call2(&JsValue::undefined(), &1.into(), &2.into())?;
    console_log!("1 + 2 = {:?}", three);

    Ok(())
}

#[wasm_bindgen(start)]
pub fn run() {
    utils::set_panic_hook();

    spawn_local(async {
        run_async().await.expect("Failed to run:");
    });
}