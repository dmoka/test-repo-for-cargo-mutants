use std::fmt;

pub struct EventBus {

}

impl EventBus {

    pub fn deposit_event(event_id: u32) {
        print!("event with id {} is emitted", event_id);
    }

    pub fn do_logic(event_id: u32) {
        //some logic
        Self::deposit_event(3);
    }
}

fn main() {
}


#[test]
fn test1() {
    assert!(true);
}