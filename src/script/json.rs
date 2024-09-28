pub fn to_json(value: mlua::Value) -> serde_json::Value {
    match value {
        mlua::Value::Boolean(v) => serde_json::Value::Bool(v),
        mlua::Value::String(v) => serde_json::Value::String(v.to_str().unwrap().to_string()),
        mlua::Value::Number(v) => serde_json::Value::Number(serde_json::Number::from_f64(v).unwrap()),
        mlua::Value::Integer(v) => serde_json::Value::Number(v.into()),
        mlua::Value::Nil => serde_json::Value::Null,
        mlua::Value::Table(v) => {
            let mut map = serde_json::Map::new();
            for pair in v.pairs::<mlua::Value, mlua::Value>() {
                let (key, value) = pair.unwrap();
                let key = if let mlua::Value::String(k) = key { k.to_str().unwrap().to_string() } else { continue };
                map.insert(key, to_json(value));
            }
            serde_json::Value::Object(map)
        },
        _ => todo!()
    }
}