// Pushdown Automata - when a state machine needs to remember it's previous states.
// - 'push' a new state down on top of old state.
// - 'pop' that state to return to previous state.

#[derive(Copy, Clone)]
pub enum State {
    Default,
    TestState(i32),
}

pub enum Switch {
    Push(State),
    Pop,
    To(State),
}

impl State {
    pub fn on_enter(&self) {
        match self {
            State::TestState(i) => println!("TestState {} Entered.", i),
            _ => {}
        }
    }
    pub fn on_exit(&self) {
        match self {
            State::TestState(i) => println!("TestState {} Exited.", i),
            _ => {}
        }
    }
    pub fn on_pause(&self) {
        match self {
            State::TestState(i) => println!("TestState {} Paused.", i),
            _ => {}
        }
    }
    pub fn on_resume(&self) {
        match self {
            State::TestState(i) => println!("TestState {} Resumed.", i),
            _ => {}
        }
    }
}

pub struct PushdownAutomata {
    // Sahil - ideally use a Vector here instead, that way it knows it's length and you
    // can just ask for the last element instead of doing this push/pop thing.
    stack: [Option<State>; 16],
}

impl PushdownAutomata {
    pub fn new() -> Self {
        PushdownAutomata { stack: [None; 16] }
    }

    // This.... is pretty usless right now, unless we had more to do within each switch statement.
    // You can do:
    // let mut pd = PushdownAutomata::new();
    //
    // pd.switch(Switch::Push(State:TestState(1)));
    // OR
    // pd.push(State::TestState(1));
    pub fn switch(&mut self, sw: Switch) {
        match sw {
            Switch::Push(next_state) => {
                self.push(next_state);
            }
            Switch::Pop => {
                self.pop();
            }
            Switch::To(next_state) => {
                self.to(next_state);
            }
        }
    }

    pub fn push(&mut self, next: State) {
        if let Some(prev) = self.stack[0] {
            prev.on_pause();
        }
        next.on_enter();
        self.shift_r();
        self.stack[0] = Some(next);
    }

    fn shift_r(&mut self) {
        let mut acc: [Option<State>; 16] = [None; 16];
        for (idx, item) in self.stack.iter().enumerate() {
            if idx + 1 < acc.len() {
                acc[idx + 1] = *item;
            }
        }
        self.stack = acc;
    }

    pub fn pop(&mut self) {
        if let Some(state) = self.stack[0] {
            state.on_exit();
            self.shift_l();
            if let Some(next) = self.stack[0] {
                next.on_resume();
            }
        }
    }

    fn shift_l(&mut self) {
        let mut acc: [Option<State>; 16] = [None; 16];
        for (idx, item) in self.stack.iter().enumerate() {
            if idx > 0 {
                acc[idx - 1] = *item;
            }
        }
        self.stack = acc;
    }

    pub fn to(&mut self, next: State) {
        if let Some(prev) = self.stack[0] {
            prev.on_exit();
        }
        self.stack = [None; 16];
        self.push(next);
    }
}
