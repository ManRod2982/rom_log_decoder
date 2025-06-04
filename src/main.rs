use std::env;
use std::fs;
use std::collections::HashMap;

struct EventData {
    description: String,
    parameters: u8,
}

impl EventData{
    // Creates a new event data
    fn new(description: &str, parameters: u8) -> EventData {
        EventData {description: description.to_string(), parameters}
    }
}


fn main() {

    // Map ROM event IDs to the events data i.e. description and number of parameters
    // TODO: Convert to a static map instead
    let rom_events_v3 = HashMap::from([
        ("01", EventData::new("ROM event version, bit[7:0] is the version", 0)),
        ("02", EventData::new("Set up the boot device driver fails", 0))
    ]);

    let args: Vec<String> = env::args().collect();

    if args.len() == 1{
        println!("Expected an argument, please call 'rom_log_decoder rom_logs_path.txt'");
        return
    }

    let rom_log_path = &args[1];

    let rom_logs = fs::read_to_string(rom_log_path)
        .expect("Unable to read the file!");

    for log in rom_logs.lines() {
        //See if log is in our events
        let id: &str = &log[..2];
        if rom_events_v3.contains_key(id) {
            let event_data = rom_events_v3.get(id);
            println!("{id}:{}", event_data.unwrap().description);
            println!("{log}");
        } else {
            println!("Unknown id!");
            println!("{log}");
        }
    }
}
