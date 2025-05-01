use crate::helper::vec_to_js_array;
use neon::prelude::*;

// 最基础的结构
pub fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello node"))
}

/// js对应的rust基本类型
/// JsNumber => f64
/// JsString => String
/// JsBoolean => bool
/// JsObject => handle
/// JsArray => handle
/// JsFunction => handle

// 列举所有传入的类型
pub fn print_args(mut cx: FunctionContext) -> JsResult<JsString> {
    let number = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let string = cx.argument::<JsString>(1)?.value(&mut cx);
    let boolean = cx.argument::<JsBoolean>(2)?.value(&mut cx);
    let object = cx.argument::<JsObject>(3)?;
    let arr = cx.argument::<JsArray>(4)?;
    let callback = cx.argument::<JsFunction>(5)?;
    println!("number is {number},string is {string},boolean is {boolean} object is {:?},arr is {:?},callback is {:?}",object,arr,callback);
    Ok(cx.string("hello arguments"))
}

// object,ar,callback类型的使用
pub fn use_object(mut cx: FunctionContext) -> JsResult<JsObject> {
    let obj = cx.argument::<JsObject>(0)?;

    Ok(obj)
}

/// 返回值
/// JsNumber => cx.number(f64)
/// JsString => cx.string("hello")
/// JsBoolean => cx.boolean(false)
/// JsNull => cx.null()
/// JsUndefined => cx.undefined()
/// ok_js_array => cx.Array
/// ok_js_object => cx.Object

// 返回字符串
pub fn return_string(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello node"))
}

// 返回数字
pub fn return_number(mut cx: FunctionContext) -> JsResult<JsNumber> {
    Ok(cx.number(100))
}

// 返回布尔
pub fn return_boolean(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    Ok(cx.boolean(true))
}

// 返回undefined
pub fn return_undefined(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    Ok(cx.undefined())
}

// 返回null
pub fn return_null(mut cx: FunctionContext) -> JsResult<JsNull> {
    Ok(cx.null())
}

// 返回简单数组
// 使用JsArray::new
pub fn return_simple_arr(mut cx: FunctionContext) -> JsResult<JsArray> {
    let values = [1, 3, 5, 7];
    let js_arr = JsArray::new(&mut cx, values.len() as u32 as usize);

    for (i, val) in values.iter().enumerate() {
        let js_val = cx.number(*val);
        js_arr.set(&mut cx, i as u32, js_val)?;
    }

    Ok(js_arr)
}

// 返回数组
pub fn return_array(mut cx: FunctionContext) -> JsResult<JsArray> {
    let arr2 = vec!["1", "2"];
    let js_arr = vec_to_js_array(&mut cx, &arr2)?;
    Ok(js_arr)
}

// 错误用法：返回简单对象
// pub fn return_object(mut cx: FunctionContext) -> JsResult<JsObject> {
//     let obj = JsObject::new(&mut cx);
//     obj.set(&mut cx, "name", cx.string("hello world"))?;
//     obj.set(&mut cx, "age", cx.number(18))?;
//
//     let user_hobby = vec!["football", "basketball"];
//     let user_hobby = vec_to_js_array(&mut cx, &user_hobby)?;
//     obj.set(&mut cx, "user_hobby", user_hobby)?;
//
//     Ok(obj)
// }
pub fn return_object(mut cx: FunctionContext) -> JsResult<JsObject> {
    let obj = JsObject::new(&mut cx);

    let name = cx.string("hello world");
    let age = cx.number(18);
    obj.set(&mut cx, "name", name)?;
    obj.set(&mut cx, "age", age)?;

    let hobbies = vec!["football", "basketball"];
    let hobby_arr = vec_to_js_array(&mut cx, &hobbies)?;
    obj.set(&mut cx, "user_hobby", hobby_arr)?;

    Ok(obj)
}


// 返回方法
fn rust_add(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let a = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let b = cx.argument::<JsNumber>(1)?.value(&mut cx);
    Ok(cx.number(a + b))
}
pub fn return_function(mut cx: FunctionContext) -> JsResult<JsFunction> {
    JsFunction::new(&mut cx, rust_add)
}