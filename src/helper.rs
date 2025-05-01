use neon::prelude::*;

/// 一个 trait，表示某种类型可以转换成 JsValue
pub trait ToJsValue<'a> {
    fn to_js_value(self, cx: &mut FunctionContext<'a>) -> Handle<'a, JsValue>;
}

impl<'a> ToJsValue<'a> for i32 {
    fn to_js_value(self, cx: &mut FunctionContext<'a>) -> Handle<'a, JsValue> {
        cx.number(self as f64).upcast()
    }
}

impl<'a> ToJsValue<'a> for f64 {
    fn to_js_value(self, cx: &mut FunctionContext<'a>) -> Handle<'a, JsValue> {
        cx.number(self).upcast()
    }
}

impl<'a> ToJsValue<'a> for &str {
    fn to_js_value(self, cx: &mut FunctionContext<'a>) -> Handle<'a, JsValue> {
        cx.string(self).upcast()
    }
}

impl<'a> ToJsValue<'a> for String {
    fn to_js_value(self, cx: &mut FunctionContext<'a>) -> Handle<'a, JsValue> {
        cx.string(self).upcast()
    }
}

impl<'a> ToJsValue<'a> for bool {
    fn to_js_value(self, cx: &mut FunctionContext<'a>) -> Handle<'a, JsValue> {
        cx.boolean(self).upcast()
    }
}

pub fn vec_to_js_array<'a, T>(cx: &mut FunctionContext<'a>, data: &[T]) -> JsResult<'a, JsArray>
where
    T: Copy + ToJsValue<'a>,
{
    let js_array = JsArray::new(cx, data.len() as u32 as usize);
    for (i, item) in data.iter().enumerate() {
        let js_value = item.to_js_value(cx);
        js_array.set(cx, i as u32, js_value)?;
    }
    Ok(js_array)
}
