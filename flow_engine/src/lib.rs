pub mod engine {
    use std::fs::read;
    use std::str;
    use extism::{Plugin, Context};

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
            pub inlets: Vec<Inlet>,
            pub outlets: Vec<Outlet>
        }

        #[derive(Serialize, Deserialize, Debug)]
        pub struct Inlet {
            pub id: String,
            pub name: String,
            pub _type: String,
            pub required: bool
        }

        #[derive(Serialize, Deserialize, Debug)]
        pub struct Outlet {
            pub id: String,
            pub name: String,
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
    }

    pub fn run_flow(state: &state::State, triggered_by: &str) {
        let mut stack: Vec<&str> = Vec::new();
        stack.push(triggered_by);
        while let Some(ptr) = stack.pop() {
            for ele in &state.graph.nodes {
                if ele.id == ptr {
                    let data = try_load_wasm_file(&ele.source).unwrap();
                    let results = try_run_wasm(data, "add", r#"{"a": 2, "b": 3}"#).unwrap();
                    println!("{results}");
                }
            }
            println!("{ptr}");
        }
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
            Ok(content) => Ok(str::from_utf8(content).unwrap().to_string()),
            Err(err) => {
                print!("error: {:?}", err);
                return Err("error".to_string());
            }
        }
    }

}
