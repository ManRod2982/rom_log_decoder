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
        ("03", EventData::new("Handling the first 8 kB data of the boot image fails", 0)),
        ("04", EventData::new("Handling the boot image fails", 0)),
        ("0F", EventData::new("Enters ROM error handling", 1)),
        ("10", EventData::new("Boot mode is Boot from Fuse", 0)),
        ("1A", EventData::new("Single boot mode is selected (7ULP, 8ULP and i.MX9 series only)", 0)),
        ("11", EventData::new("Boot mode is Serial Download", 0)),
        ("12", EventData::new("Boot mode is Internal Boot", 0)),
        ("13", EventData::new("Boot mode is Test mode", 0)),
        ("1F", EventData::new("Raw boot mode setting in OCOTP fuses Bit[23:0] == Raw boot mode setting", 0)),
        ("18", EventData::new("LP boot mode is selected (7ULP, 8ULP, i.MX9 series only)", 0)),
        ("19", EventData::new("Dual boot mode is selected(7ULP, 8ULP only)", 0)),
        ("20", EventData::new("Secure config is FAB", 0)),
        ("21", EventData::new("Secure config is Field return", 0)),
        ("22", EventData::new("Secure config is open", 0)),
        ("23", EventData::new("Secure config is Closed", 0)),
        ("30", EventData::new("Internal use", 0)),
        ("31", EventData::new("Internal use", 0)),
        ("40", EventData::new("FUSE_SEL_VALUE Fuse is not blown", 0)),
        ("41", EventData::new("FUSE_SEL_VALUE Fuse is blown", 0)),
        ("50", EventData::new("Boot from the primary boot image", 0)),
        ("51", EventData::new("Boot from the secondary boot image", 0)),
        ("52", EventData::new("Boot from the recovery boot image", 0)),
        ("53", EventData::new("Boot via USB serial download", 0)),
        ("60", EventData::new("Primary boot from RAW NAND device", 0)),
        ("61", EventData::new("Primary boot from SD or EMMC device", 0)),
        ("6E", EventData::new("No valid primary boot device is selected via boot cfg pins/fuses", 0)),
        ("66", EventData::new("Primary boot from one NAND device", 0)),
        ("67", EventData::new("Primary boot from QSPI NOR device", 0)),
        ("68", EventData::new("EMMC fast boot is selected via fuse (8ULP, i.MX9 series)", 0)),
        ("72", EventData::new("No recovery boot device", 0)),
        ("73", EventData::new("Recovery boot from SD or EMMC", 0)),
        ("75", EventData::new("Recovery boot from FlexSPI NOR", 0)),
        ("80", EventData::new("Start to perform device initialization", 1)),
        ("81", EventData::new("The boot device initialization completes", 1)),
        ("82", EventData::new("Starts to execute Boot device driver pre-config", 0)),
        ("83", EventData::new("Boot device driver pre-config completes", 0)),
        ("84", EventData::new("Boot image set 0 in the primary boot device is selected", 0)),
        ("85", EventData::new("Boot image set 1 in the primary boot device is selected", 0)),
        ("86", EventData::new("The offset of boot image set1 is valid", 0)),
        ("8D", EventData::new("Both boot images set0 and set1 are all invalid", 0)),
        ("8E", EventData::new("Boot device driver pre-config fails", 0)),
        ("8F", EventData::new("The boot device initialization fails", 1)),
        ("90", EventData::new("Start to read dat from boot device", 2)),
        ("91", EventData::new("Reading data from a boot device completes", 1)),
        ("94", EventData::new("The one in the core image target is in FlexSPI NOR space runs with XIP mode", 0)),
        ("9E", EventData::new("The target space of the boot image recorded in container header is not valid", 2)),
        ("9F", EventData::new("Reading data from boot device fails", 1)),
        ("A0", EventData::new("Image authentication result", 2)),
        ("A1", EventData::new("SECO container header is not valid", 0)),
        ("A2", EventData::new("SECO container header is valid", 0)),
        ("A3", EventData::new("SECO FW authentication pass", 2)),
        ("A4", EventData::new("SECO FW authentication fails", 2)),
        ("A5", EventData::new("SCU container authentication pass", 2)),
        ("A6", EventData::new("SCU container authentication fails", 2)),
        ("A7", EventData::new("The image verify passes", 3)),
        ("A8", EventData::new("The image verify fails", 3)),
        ("A9", EventData::new("Release the container done", 1)),
        ("AA", EventData::new("Release the container fails", 1)),
        ("AB", EventData::new("DDR script available", 0)),
        ("AC", EventData::new("SCU container header is not valid", 0)),
        ("AD", EventData::new("SCU container header is valid", 0)),
        ("AE", EventData::new("V2X container authentication pass (DXL only)", 2)),
        ("AF", EventData::new("V2X container authentication fails (DXL only)", 2)),
        ("B0", EventData::new("Starts to execute DDR script", 2)),
        ("B1", EventData::new("Running DDR script completes", 1)),
        ("BA", EventData::new("Enhance image verify pass (DXL only)", 0)),
        ("BF", EventData::new("DDR script returns failure", 1)),
        ("C0", EventData::new("Jump to the boot image soon", 3)),
        ("CF", EventData::new("SCFW unexpectedly returns back to ROM code", 0)),
        ("D0", EventData::new("Enters serial download processing", 0)),
        ("E0", EventData::new("Internal use", 0)),
        ("F0", EventData::new("Enters ROM exception handler", 1)),
        ("F1", EventData::new("Switch boot stage", 2)),
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
