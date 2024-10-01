use std::{fs, path::PathBuf};
use json::to_json;
use mlua::{Lua, Table};

use crate::Result;

pub mod json;

fn register<'a>(lua: &'a Lua, namespace: &mut Table<'a>) -> Result<()> {
    namespace.set("prompt_text", lua.create_function(|_, message: String| {
        Ok(inquire::Text::new(&message)
            .prompt()
            .unwrap())
    })?)?;

    namespace.set("prompt_select", lua.create_function(|_, (message, options): (String, Vec<String>)| {
        Ok(inquire::Select::new(&message, options)
            .prompt()
            .unwrap())
    })?)?;

    Ok(())
}

pub fn eval_from(path: PathBuf) -> Result<serde_json::Value> {
    // 1. Read content and init lua
    let content = fs::read_to_string(&path)?;

    let lua = mlua::Lua::new();

    // 2. Register namespace variables
    let mut namespace = lua.create_table()?;
    register(&lua, &mut namespace)?;
    lua.globals().set("forge", namespace)?;

    // 3. Execute and return
    let result = lua.load(content).eval()?;
    Ok(to_json(result))
}

pub fn eval_from_all(paths: Vec<PathBuf>) -> Vec<serde_json::Value> {
    paths.into_iter()
        .map(|path| eval_from(path))
        .map(|path| path.unwrap())
        .collect()
}