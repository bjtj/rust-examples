# reqwest

<https://docs.rs/reqwest/latest/reqwest/>

example

``` rust
let body = reqwest::get("https://www.rust-lang.org")
    .await?
    .text()
    .await?;

println!("body = {body:?}");
```

