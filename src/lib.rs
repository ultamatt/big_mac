use neon::prelude::*;
use mac_address::{ get_mac_address, mac_address_by_name };
const default_mac:& str = "00:00:00:00:00:00";

fn get_mac(mut cx: FunctionContext) -> JsResult<JsString> {
    let results = match get_mac_address() {
        Ok(Some(ma)) => Some(ma),
        Ok(None) => None,
        Err(e) => None,
    };
    
    match results {
        Some(ma) => {
            return Ok(cx.string(ma.to_string()))
        },
        None => {
            //panic!("No Mac dude");
            return Ok(cx.string(& default_mac))
        },
    };
}

fn get_mac_by_name(mut cx: FunctionContext) -> JsResult<JsString> {
    let da_name = cx.argument::<JsString>(0)?.value(&mut cx);
    let results = match mac_address_by_name(&da_name) {
        Ok(Some(ma)) => Some(ma), 
        Ok(None) => None, 
        Err(e) => None,
    };
    
    match results {
        Some(ma) => {
            return Ok(cx.string(ma.to_string()))
        }, 
        None => {
            //panic!(String::from("No Mac or Invalid Interface"));
            return Ok(cx.string(& default_mac))
        }
    };
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("get_mac_by_name", get_mac_by_name);
    cx.export_function("get_mac", get_mac)?;
    Ok(())
}
