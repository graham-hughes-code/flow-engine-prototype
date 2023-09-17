use flow_engine::engine::state::State;
use flow_engine::engine::{run_flow, get_node_frontend, get_node_def};
use std::fs::read_to_string;

fn main() {

    let font_end = get_node_frontend("std/constant.wasm").unwrap();
    println!("frontend: {:?}", font_end);

    let node_def = get_node_def("std/constant.wasm").unwrap();
    println!("node def: {:?}", node_def);

    let state_file: String = read_to_string("example-graph.json").unwrap().parse().unwrap();
    let mut state: State = State::from_str(&state_file).unwrap();
    run_flow(&mut state, "n_tysfsaddfgjgff");
    println!("{:?}", state);
}
