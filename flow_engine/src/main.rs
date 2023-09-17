use flow_engine::engine::state::State;
use flow_engine::engine::{run_flow, get_node_frontend};
use std::fs::read_to_string;

fn main() {
    let state_file: String = read_to_string("example-graph.json").unwrap().parse().unwrap();
    let mut state: State = State::from_str(&state_file).unwrap();
    run_flow(&mut state, "n_tysfsaddfgjgff");
    let font_end = get_node_frontend("std/constant.wasm").unwrap();
    print!("{:?}", font_end);
    println!("{:?}", state);
}
