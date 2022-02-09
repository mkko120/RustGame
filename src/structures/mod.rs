use druid::{Data, Lens};

#[derive(Clone, Data, Lens)]
pub struct HelloState {
    pub name: String,
}

pub fn p() {
    println!("Hello from HelloState");
}
