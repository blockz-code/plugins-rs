use deno_ast::swc::common::sync::Send;
use deno_core::v8;
use deno_core::{serde_v8, v8::Local};
use deno_core::{ extension, op2, OpState };

use deno_error::JsErrorBox;
use ffbins_rs::{Binary, FFbins, State, Version};




#[op2(fast)]
fn instance(
    state: &mut OpState,
) -> std::io::Result<()> {
    state.put(FFbins::new(Binary::FFmpeg, Version::V8_0_1, "/usr/local/bin/".into(), "/tmp/".into()).init().map_err(|e| JsErrorBox::generic(e.to_string())));
    Ok(())
}



fn ffbins_cb<T>(op_state: &mut OpState, cb: T) -> crate::Result<()>
where
    T: Fn(State, u64, u64, f64) + Send + Sync + std::marker::Send,
{
    let mut ffbins: FFbins = op_state.take();

    ffbins.install(move |state, current, total, percent| {

        cb(state, current, total, percent);

    }).unwrap();

    Ok(())
}


#[op2]
#[serde]
fn install<'s>(
    scope: &mut v8::PinScope<'s, '_>,
    op_state: &mut OpState,
    //#[serde] callback: serde_v8::Value,
    #[scoped] callback: v8::Global<v8::Function>,
) -> std::io::Result<()> {

    let ocb = callback.open(scope);

    let _empty: Local<v8::String> = to_local(scope, "").into();
    let context = v8::Context::new(scope, Default::default());
    let scope = &mut v8::ContextScope::new(scope, context);

    let recv: Local<v8::String> = to_local(scope, "").into();

    let (tx, rx) = std::sync::mpsc::channel::<(State, u64, u64, f64)>();

    ffbins_cb(op_state, |state, current, total, percent| {

        tx.send((state, current, total, percent)).unwrap();

    }).unwrap();

    for (state, current, total, percent) in rx.iter() {

        let args = &[
            v8::String::new(scope, &state.to_string()).unwrap().into(),
            v8::Number::new(scope, current as f64).into(),
            v8::Number::new(scope, total as f64).into(),
            v8::Number::new(scope, percent).into(),
        ];

        ocb.call_with_context(scope, context, recv.into(), args).unwrap();

    }

    Ok(())
}


// #[op2(reentrant)]



fn to_local<'s>(scope: &mut v8::PinScope<'s, '_>, s: &str) -> Local<'s, v8::String> {
    v8::String::new(scope, s).unwrap()
}

#[inline]
#[allow(unused)]
fn to_v8_fn(
    scope: &mut v8::Isolate,
    value: serde_v8::Value,
) -> crate::Result<v8::Global<v8::Function>> {
    v8::Local::<v8::Function>::try_from(value.v8_value)
        .map(|cb| v8::Global::new(scope, cb))
        .map_err(|err| crate::Error::Unknown(err.to_string()))
}



#[inline]
#[allow(unused)]
fn to_v8_local_fn(
    value: serde_v8::Value,
) -> crate::Result<v8::Local<v8::Function>> {
    v8::Local::<v8::Function>::try_from(value.v8_value)
        .map_err(|err| crate::Error::Unknown(err.to_string()))
}




extension!(
    media_js,
    ops = [
        instance,
        install,
    ],
    esm_entry_point = "plugins-rs:media", 
    esm = [
        dir "src/extensions/media",
        "plugins-rs:media" = "index.js"
    ],
    docs = "Rust Based HTML Scraper", "scraper html from rust"
);





pub fn init() -> deno_core::Extension {
    media_js::init()
}
