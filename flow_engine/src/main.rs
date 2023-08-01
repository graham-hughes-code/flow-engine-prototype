use flow_engine::engine::state::State;
use flow_engine::engine::run_flow;
use std::fs::read_to_string;

fn main() {
    let state_file: String = read_to_string("state.json").unwrap().parse().unwrap();
    let state: State = State::from_str(&state_file).unwrap();
    run_flow(&state, "n_asdfasdfg234f");
    println!("{:?}", state);
}
