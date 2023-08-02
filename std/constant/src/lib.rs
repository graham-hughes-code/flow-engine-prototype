use extism_pdk::*;
use serde::Serialize;

#[derive(Serialize)]
enum OutputValue {
    S(String),
    N(f64)
}

#[derive(Serialize)]
struct Output {
    pub a: OutputValue
}

#[plugin_fn]
pub fn add(input: String) -> FnResult<Json<Output>> {
    Ok(Json(Output { a: OutputValue::S(String::from("hello world")) }))
}


#[plugin_fn]
pub fn describe_node(_: ()) -> FnResult<String>
{
    let dis: String =
        r#"{"inputs": {},
            "Output": {
                "c": {
                    "type": ["Number", "String"]
                }
            }
        }"#.to_string();

    Ok(dis)
}
