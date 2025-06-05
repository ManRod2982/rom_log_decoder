use std::env;
use std::fs;
use std::collections::HashMap;
use std::io::prelude::*;

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


fn main() -> std::io::Result<()> {

    // Map ROM event IDs to the events data i.e. description and number of parameters
    // TODO: Convert to a static map instead
    let rom_events_v3 = HashMap::from([
        ("01", EventData::new("ROM event version, bit[7:0] is the version", 0)),
        ("02", EventData::new("Set up the boot device driver fails", 0)),
        ("0F", EventData::new("Enters ROM error handling", 1)),
    ]);

    let args: Vec<String> = env::args().collect();

    if args.len() <= 2{
        println!("Expected an argument, please call 'rom_log_decoder rom_logs_path.txt'");
        return Ok(())
    }

    let rom_log_path = &args[1];
    let decode_log_path =  &args[2];

    let rom_logs = fs::read_to_string(rom_log_path)
        .expect("Unable to read the file!");

    let mut decode_file = fs::File::create(decode_log_path)?;

    // Get iterator, cannot use for loop since we need to iterate inside the loop
    // to process parameters
    let mut rom_lines = rom_logs.lines();

    while let Some(log) = rom_lines.next() {
        //See if log is in our events
        let id: &str = &log[..2];
        if rom_events_v3.contains_key(id) {
            let event_data = rom_events_v3.get(id);
            writeln!(decode_file, "Event ID:{id}-{}, {} parameters", event_data.unwrap().description, event_data.unwrap().parameters)?;
            writeln!(decode_file, "{log}")?;
            // Process parameters if any
            for param in 0..event_data.unwrap().parameters {
                writeln!(decode_file, "Parameter{param}")?;
                if let Some(p) = rom_lines.next() {
                    writeln!(decode_file, "{}", p)?;
                }
            }
        } else {
            writeln!(decode_file, "Unknown id!")?;
            writeln!(decode_file, "{log}")?;
        }
        writeln!(decode_file, " ")?;
    }
    Ok(())
}
