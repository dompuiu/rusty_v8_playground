// https://stackoverflow.com/questions/53497160/native-v8promise-result
// https://github.com/danbev/learning-v8/blob/master/rusty-v8/src/main.rs
// https://github.com/nodejs/node/issues/5691
// https://docs.google.com/document/d/1rda3yKGHimKIhg5YeoAmCOtyURgsbTH_qaYR79FELlk/edit
// https://v8.dev/docs/embed

use color_eyre::eyre::Result;
use std::fs::File;
use std::io::prelude::*;
use std::{convert::TryInto, time};
use tokio::time::{sleep, Duration};

// #[tokio::main]
fn main() -> Result<()> {
    initialize_v8();
    let code_from_file = read_code()?;
    run_code(code_from_file);

    unsafe {
        v8::V8::dispose();
    }
    v8::V8::dispose_platform();
    Ok(())
}

fn initialize_v8() {
    let platform = v8::new_default_platform(0, false).make_shared();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();
}

fn run_code(code_from_file: String) {
    let isolate = &mut v8::Isolate::new(Default::default());

    {
        let handle_scope = &mut v8::HandleScope::new(isolate);
        let context = v8::Context::new(handle_scope);
        let scope = &mut v8::ContextScope::new(handle_scope, context);

        // Console
        let console_object_template = v8::ObjectTemplate::new(scope);
        console_object_template.set(
            v8::String::new(scope, "log").unwrap().into(),
            v8::FunctionTemplate::new(scope, print).into(),
        );

        // Add Console Method inside v8.
        let global = v8::ObjectTemplate::new(scope);
        global.set(
            v8::String::new(scope, "console2").unwrap().into(),
            console_object_template.into(),
        );

        global.set(
            v8::String::new(scope, "setTimeout").unwrap().into(),
            v8::FunctionTemplate::new(scope, set_timeout).into(),
        );

        let context = v8::Context::new_from_template(scope, global);
        let scope = &mut v8::ContextScope::new(scope, context);

        let code = v8::String::new(scope, &code_from_file).unwrap();

        let script = v8::Script::compile(scope, code, None).unwrap();
        let x = script.run(scope);
        let result = x.unwrap();
        // let sresult = result.to_string(scope).unwrap();
        // println!("result: {}", sresult.to_rust_string_lossy(scope));

        let promise: Result<v8::Local<v8::Promise>, _> = result.try_into();

        let x = promise.unwrap();
        println!("promise state: {:?}", x.state());
        println!(
            "{:?}",
            x.result(scope)
                .to_string(scope)
                .unwrap()
                .to_rust_string_lossy(scope)
        );

        // println!("result: {:?}", result.is_promise());
    }
}

fn read_code() -> std::io::Result<String> {
    let mut file = File::open("src/code.js")?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn print(scope: &mut v8::HandleScope, args: v8::FunctionCallbackArguments, _: v8::ReturnValue) {
    let result = args
        .get(0)
        .to_string(scope)
        .unwrap()
        .to_rust_string_lossy(scope);

    println!("console.log: {}", result);
}

fn set_timeout(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    _: v8::ReturnValue,
) {
    let timeout_fn = args.get(0).is_function();
    let timeout = args.get(1).integer_value(scope).unwrap();

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        sleep(Duration::from_millis(timeout.try_into().unwrap())).await;
        println!("{} ms have elapsed {}", timeout, timeout_fn);
    });
}
