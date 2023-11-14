pub type State = usize;

pub trait NewState {
    fn new() -> Self;
}

impl NewState for usize {
    fn new() -> Self {
        0
    }
}

impl NewState for String {
    fn new() -> Self {
        String::from("0")
    }
}

impl NewState for char {
    fn new() -> Self {
        '0'
    }
}
