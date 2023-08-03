use flow_engine::engine::state::State;
use flow_engine::engine::run_flow;
use std::fs::read_to_string;

fn main() {
    let state_file: String = read_to_string("state.json").unwrap().parse().unwrap();
    let mut state: State = State::from_str(&state_file).unwrap();
    state = run_flow(state, "n_tysfsaddfgjgff");
    println!("{:?}", state);
}
