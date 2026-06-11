# Network information

## hostname

<https://docs.rs/hostname/0.4.2/hostname/>

e.g.)

``` rust
use std::io;

fn main() -> io::Result<()> {
    // Retrieve the hostname
    dbg!(hostname::get()?);

    // And set a new one
    hostname::set("potato")?;

    Ok(())
}
```

## network-interface

<https://docs.rs/network-interface/2.0.5/network_interface/>
