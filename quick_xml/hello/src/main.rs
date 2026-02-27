use std::fs;
use quick_xml::events::Event;
use quick_xml::Reader;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let xml = fs::read_to_string("example.xml")?;

    let mut reader = Reader::from_str(&xml);
    reader.config_mut().trim_text(true);

    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                println!("Start tag: {}", String::from_utf8_lossy(e.name().as_ref()));
            }
            Ok(Event::Text(e)) => {
                println!("Text: {}", e.decode().expect("Failed to decode UTF-8"));
            }
            Ok(Event::End(e)) => {
                println!("End tag: {}", String::from_utf8_lossy(e.name().as_ref()));
            }
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error: {:?}", e),
            _ => {}
        }
        buf.clear();
    }

    Ok(())
}
