// https://medium.com/swlh/rust-pushdown-automata-d37c2b1ae0c6
mod pushdown;
use pushdown::*;

fn main() {
    let mut pd = PushdownAutomata::new();

    // Since `pd.switch()` isn't doing any extra logic, you can either use or not use it.
    pd.push(State::TestState(1));
    pd.switch(Switch::Push(State::TestState(2)));
    pd.pop();
    pd.switch(Switch::To(State::TestState(3)));
    pd.switch(Switch::Pop);
}
