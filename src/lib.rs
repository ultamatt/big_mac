use neon::prelude::*;
use mac_address::get_mac_address;
// 
// fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
//     Ok(cx.string("hello node"))
// }

fn get_mac(mut cx: FunctionContext) -> JsResult<JsString> {
    let results = match get_mac_address() {
        Ok(Some(ma)) => Some(ma),
        Ok(None) => None,
        Err(e) => None,
    };
    let returnable = match results {
        Some(ma) => ma.to_string(),
        None => String::from("No Mac"),
    };
    Ok(cx.string(returnable))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    // cx.export_function("hello", hello)?;
    cx.export_function("get_mac", get_mac)?;
    Ok(())
}
