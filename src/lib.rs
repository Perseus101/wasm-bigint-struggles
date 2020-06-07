use js_sys::{Function, Reflect, WebAssembly};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::{spawn_local, JsFuture};

mod utils;
pub mod imports;

// lifted from the `console_log` example
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(a: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

const WASM: &[u8] = include_bytes!("api_test.wasm");

async fn run_async() -> Result<(), JsValue> {
    console_log!("instantiating a new wasm module directly");
    console_log!("creating imports");

    // let native_add_closure = Closure::wrap(Box::new(native_add) as Box<dyn Fn(u64, u64) -> u64>);
    // let native_add: &Function = native_add_closure.as_ref().unchecked_ref();
    // let native_add_name = "native_add".into();

    // let env = Map::new();

    // env.set(&native_add_name, native_add.as_ref());

    // let imports = Map::new();

    // imports.set(&JsValue::from("env"), &imports::Imports::new().into());
    // let imports = Object::from(imports);

    let imports = imports::make_imports();

    console_log!("instantiating module");
    console_log!("Imports: {:?}", imports);
    let a = JsFuture::from(WebAssembly::instantiate_buffer(WASM, &imports.into())).await?;
    let b: WebAssembly::Instance = Reflect::get(&a, &"instance".into())?.dyn_into()?;

    let c = b.exports();

    let get_timestamp = Reflect::get(c.as_ref(), &"get_timestamp".into())?
        .dyn_into::<Function>()
        .expect("get_timestamp export wasn't a function");

    let three = get_timestamp.call0(&JsValue::undefined())?;
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