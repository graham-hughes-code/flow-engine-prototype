pub mod engine {
    use std::fs::read;
    use std::str;
    use extism::{Plugin, Context};
    use serde_json::Value;
    use serde::{Deserialize, Serialize};

    pub mod state {
        use serde::{Deserialize, Serialize};
        use serde_json::Value;
        use std::collections::HashMap;


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

            pub fn clear_last_values(&mut self) {
                for edge in &mut self.graph.edges{
                    edge.last_value = None;
                }
            }

            pub fn push_values_to_edges(&mut self, node_id: &str, results: &str) {
                let mut outlet_to_result: HashMap<String, Value>  = HashMap::new();

                let current_node: &Node = self.try_find_node(node_id).unwrap();

                for outlet in &current_node.outlets {

                    let results_value: Value = serde_json::from_str(results).unwrap();
                    let value_to_push: &Value = &results_value[&outlet.name];

                    outlet_to_result.insert(
                        outlet.id.clone(),
                        value_to_push.clone()
                    );
                }

                for (outlet_id, value_to_push) in outlet_to_result{
                    for edge in &mut self.graph.edges {
                        if edge.start == outlet_id {
                            edge.last_value = Some(serde_json::to_string(&value_to_push).unwrap());
                        }
                    }
                }

            }

            pub fn try_find_node_next_ids(&self, node_id: &str) -> Option<Vec<String>> {
                let mut next_node_ids: Vec<String> = Vec::new();

                let current_node: &Node = self.try_find_node(node_id).unwrap();
                let mut current_node_outlet_id: Vec<&str> = Vec::new();
                for outlet in &current_node.outlets {
                    current_node_outlet_id.push(&outlet.id);
                }

                let mut inlet_ids: Vec<&str> = Vec::new();
                for edge in &self.graph.edges {
                    if current_node_outlet_id.iter().any(|&i| i == edge.start) {
                        inlet_ids.push(&edge.end);
                    }
                }

                for node in &self.graph.nodes {
                    for inlet in &node.inlets {
                        if inlet_ids.iter().any(|&i| i == inlet.id) {
                            next_node_ids.push(node.id.clone());
                        }
                    }
                }

                if next_node_ids.is_empty() {
                    return None;
                }

                Some(next_node_ids)
            }

            pub fn try_find_node(&self, node_id: &str) -> Option<&Node> {
                for ele in &self.graph.nodes {
                    if ele.id == node_id {
                        return Some(&ele);
                    }
                }
                None
            }
        }
    }

    pub fn run_flow(state: &mut state::State, triggered_by: &str){

        state.clear_last_values();

        let mut stack: Vec<String> = Vec::new();
        stack.push(triggered_by.into());

        while let Some(ptr) = stack.pop() {

            let current_node: &state::Node = state.try_find_node(&ptr).unwrap();
            println!("{ptr}");

            // TODO: check all inlets and push to stack if not last_value
            let current_context_json: Value = pull_context(&current_node.context);
            // TODO: combine all inlets and context to create input

            let data: Vec<u8> = try_load_wasm_file(&current_node.source).unwrap();
            let results: String = try_run_wasm(
                data,
                &current_node.name,
                &serde_json::to_string(&current_context_json).unwrap()
            ).unwrap();
            state.push_values_to_edges(&ptr, &results);

            let mut node_ids = state.try_find_node_next_ids(&ptr).unwrap();
            stack.append(&mut node_ids);

            println!("{results}");

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
