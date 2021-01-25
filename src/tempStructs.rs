use std::collections::HashMap;

struct Month {
    days: HashMap<u8, Day>
}

struct Day {
    day: u8,
    events: Vec<Event>
}

struct Event {
    name: String
    
}
