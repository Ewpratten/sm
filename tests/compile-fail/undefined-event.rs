#[macro_use]
extern crate sm;

sm!{
    Lock { Locked }
}

fn main() {
    use Lock::*;
    let sm = Machine::new(Locked);

    sm.event(Invalid);
    //~^ ERROR no method named `event` found for type `Lock::Machine<Lock::Locked>` in the current scope
    //~^^ ERROR cannot find value `Invalid` in this scope
}
