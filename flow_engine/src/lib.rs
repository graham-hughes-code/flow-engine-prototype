pub mod engine {
    use std::fs::read;
    use std::str;
    use extism::{Plugin, Context};
    use serde_json::Value;
    use serde::{Deserialize, Serialize};

    pub mod state {
        use serde::{Deserialize, Serialize};

        #[derive(Serialize, Deserialize, Debug)]
        pub struct State {
            pub version: String,
            pub graph: Graph
        }

        #[derive(Serialize, Deserialize, Debug)]
        pub struct Graph {
            pub nodes: Vec<Node>,
            pub edges: Vec<Edge>
        }

        #[derive(Serialize, Deserialize, Debug)]
        pub struct Node {
            pub id: String,
            pub name: String,
            pub source: String,
            pub context: String,
            pub inlets: Vec<Inlet>,
            pub outlets: Vec<Outlet>
        }

        #[derive(Serialize, Deserialize, Debug)]
        #[serde(rename(serialize = "ser_name"))]
        pub struct Inlet {
            pub id: String,
            pub name: String,
            #[serde(rename = "type")]
            pub _type: String,
            pub required: bool
        }

        #[derive(Serialize, Deserialize, Debug)]
        pub struct Outlet {
            pub id: String,
            pub name: String,
            #[serde(rename = "type")]
            pub _type: String
        }

        #[derive(Serialize, Deserialize, Debug)]
        pub struct Edge {
            pub id: String,
            pub start: String,
            pub end: String,
            pub last_value: Option<String>
        }

        impl State {
            pub fn from_str(s: &str) -> Result<Self, String> {
                match serde_json::from_str(s) {
                    Ok(state) => Ok(state),
                    Err(e) => Err(format!("Error malformed state json {:?}", e))
                }
            }
        }

        pub fn try_find_node<'a>(nodes: &'a Vec<Node>, node_id: &str) -> Option<&'a Node> {
            for ele in nodes {
                if ele.id == node_id {
                    return Some(ele);
                }
            }
            None
        }
    }

    pub fn run_flow(state: state::State, triggered_by: &str) -> state::State {
        let mut stack: Vec<&str> = Vec::new();
        stack.push(triggered_by);

        while let Some(ptr) = stack.pop() {

            let current_node: &state::Node = state::try_find_node(&state.graph.nodes, ptr).unwrap();

            let data: Vec<u8> = try_load_wasm_file(&current_node.source).unwrap();
            let current_context_json: Value =  pull_context(&current_node.context);
            let results: String = try_run_wasm(data, &current_node.name, &serde_json::to_string(&current_context_json).unwrap()).unwrap();
            println!("{results}");

            println!("{ptr}");
        }

        state
    }

    fn try_load_wasm_file(file_path: &str) -> Result<Vec<u8>, String> {
        match read(file_path) {
            Ok(data) => Ok(data),
            Err(e) => Err(format!("Error trying to load file {file_path} {e}"))
        }
    }

    fn try_run_wasm(data: Vec<u8>, name: &str, input: &str) -> Result<String, String> {
        let context: Context = Context::new();
        let mut plugin = Plugin::new(&context, data, [], false).unwrap();
        match plugin.call(&name, &input) {
            Ok(result) => Ok(str::from_utf8(result).unwrap().to_string()),
            Err(err) => {
                print!("error: {:?}", err);
                return Err("error".to_string());
            }
        }
    }


    #[derive(Serialize, Deserialize, Debug)]
    struct ContextWrapper {
        pub context: Value
    }

    fn pull_context(con: &str) -> Value {
        serde_json::value::to_value(ContextWrapper {
            context: serde_json::from_str( con ).unwrap()
        }).unwrap()
    }

    fn merge(a: &mut Value, b: &Value) {
        match (a, b) {
            (&mut Value::Object(ref mut a), &Value::Object(ref b)) => {
                for (k, v) in b {
                    merge(a.entry(k.clone()).or_insert(Value::Null), v);
                }
            }
            (a, b) => {
                *a = b.clone();
            }
        }
    }

}
