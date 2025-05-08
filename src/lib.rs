mod function_context;
mod helper;
mod utils;

use crate::function_context::*;
use crate::utils::trace::logger_execute;
use neon::prelude::*;

fn log(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let logger_content = cx.argument::<JsString>(0)?.value(&mut cx) as String;
    utils::logger::logger_execute(logger_content);
    Ok(cx.undefined())
}

pub fn print(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let module = cx.argument::<JsString>(0)?.value(&mut cx);
    let content = cx.argument::<JsString>(1)?.value(&mut cx);
    logger_execute(module, content);
    Ok(cx.undefined())
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("hello", hello)?;
    cx.export_function("printArgs", print_args)?;
    cx.export_function("readArray", read_array)?;
    cx.export_function("readObject", read_object)?;
    cx.export_function("readFunction", read_function)?;

    cx.export_function("returnString", return_string)?;
    cx.export_function("returnNumber", return_number)?;
    cx.export_function("returnBoolean", return_boolean)?;
    cx.export_function("returnUndefined", return_undefined)?;
    cx.export_function("returnNull", return_null)?;
    cx.export_function("returnSimpleArr", return_simple_arr)?;
    cx.export_function("returnArray", return_array)?;
    cx.export_function("returnObject", return_object)?;
    cx.export_function("returnFunction", return_function)?;

    cx.export_function("print", print)?;

    Ok(())
}
