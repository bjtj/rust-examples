use std::collections::HashMap;

fn main() {
    test1();
    test2();
    test3();
}

fn test1() {
    parse_and_print("http://localhost/a/b?t=123");
    parse_and_print("http://localhost/a/b");
}

fn test2() {
    let hello = String::from("Hello, world!");
    println!("{hello}");
    let mut hello = String::from("Hello, ");
    hello.push('w');
    hello.push_str("orld!");
    println!("{hello}");
}

fn test3() {
    let sparkle_heart = vec![240, 159, 146, 150];
    let sparkle_heart = String::from_utf8(sparkle_heart).unwrap();
    assert_eq!("💖", sparkle_heart);
}

fn parse_and_print(uri: &str) {
    match parse(uri) {
        Ok(parsed) => {
            println!("path: {:?}, time: {:?}", parsed.path, parsed.time);
        }
        Err(_) => {}
    }
}

#[derive(Debug)]
struct ParsedA {
    path: String,
    time: Option<String>
}

fn parse(uri: &str) -> Result<ParsedA, ()> {
    const PREFIX:&str = "http://localhost";
    if let Some(fullquery) = uri.strip_prefix(PREFIX) {
        let (path, time) = match fullquery.split_once('?') {
            Some((path, query)) => {
                let params = parse_query(query);
                let time = params.get("t");
                (
                    path.to_string(),
                    time.cloned(),
                )
            },
            None => (
                fullquery.to_string(),
                None,
            )
        };
        return Ok(ParsedA {
            path: path,
            time: time
        });
    }
    Err(())
}

fn parse_query(query: &str) -> HashMap<String, String> {
    query
        .trim_start_matches('?')
        .split('&')
        .filter_map(|pair| {
            let (key, value) = pair.split_once('=')?;
            Some((key.to_string(), value.to_string()))
        })
        .collect()
        
}
