use std::sync::Arc;

use hematita::{ast::{lexer::Lexer, parser}, compiler, lua_lib::{self}, lua_tuple, lua_value, vm::{value::{Table, Value}, VirtualMachine}};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = document)]
    fn write(text: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(message: &str);
    fn alert(message: &str);

}

#[wasm_bindgen]
pub fn execute_lua(source: &str) {
    /* print - console.log */
    let print = |arguments: Arc<Table>, _: &_| {
        let args: Vec<String> = arguments.data
        .lock()
        .unwrap()
        .iter()
        .filter_map(|arg| {
            if arg.1.string().is_none() {
                return None;
            }
            Some(format!("{}", arg.1.string().unwrap()))
        })
        .collect();

        log(&args[0]);
        Ok(lua_tuple![].arc())
    };

    /* alert */
    let lua_alert = |arguments: Arc<Table>, _: &_| {
        let args: Vec<String> = arguments.data
        .lock()
        .unwrap()
        .iter()
        .filter_map(|arg| {
            if arg.1.string().is_none() {
                return None;
            }
            Some(format!("{}", arg.1.string().unwrap()))
        })
        .collect();

        alert(&args[0]);
        Ok(lua_tuple![].arc())
    };

    let lua_doc_write = |arguments: Arc<Table>, _: &_| {
        let args: Vec<String> = arguments.data
        .lock()
        .unwrap()
        .iter()
        .filter_map(|arg| {
            if arg.1.string().is_none() {
                return None;
            }
            Some(format!("{}", arg.1.string().unwrap()))
        })
        .collect();

        write(&args[0]);
        Ok(lua_tuple![].arc())
    };

    let document_table = Table::default();
    {
        let mut document_data = document_table.data.lock().unwrap();
        document_data.insert(lua_value!("write"), Value::NativeFunction(&lua_doc_write));
    }

    let global = {
        let global_ctx = lua_lib::standard_globals();
        let mut table = global_ctx.data.lock().unwrap();

        table.insert(lua_value!("document"), Value::Table(Arc::new(document_table)));
        table.insert(lua_value!("print"), Value::NativeFunction(&print));
        table.insert(lua_value!("alert"), Value::NativeFunction(&lua_alert));
        
        drop(table);
        global_ctx
    };

    let lua_vm = VirtualMachine::new(global);

    let lexer = Lexer { source: source.chars().peekable() }.peekable();
    let parsed = parser::parse_block(&mut parser::TokenIterator(lexer)).unwrap();
    let compiled_bytecode = compiler::compile_block(&parsed);

    lua_vm.execute(
        &compiled_bytecode.into(),
        lua_tuple![].arc()
    )
    .expect("Error while executing lua");
}