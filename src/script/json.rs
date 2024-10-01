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

pub fn merge_json(a: &mut serde_json::Value, b: serde_json::Value) {
    if let serde_json::Value::Object(a) = a {
        if let serde_json::Value::Object(b) = b {
            for (k, v) in b {
                if v.is_null() {
                    a.remove(&k);
                } else {
                    merge_json(a.entry(k).or_insert(serde_json::Value::Null), v);
                }
            }

            return;
        }
    }

    *a = b;
}