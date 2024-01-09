# wordstat-rs 

Rust library for interacting with the Yandex Wordstat API

Usage:
```rust
let client = Client::new("token", "https://api-sandbox.direct.yandex.ru/v4/json/");
let regions = get_regions(&client).await.unwrap();
println!("{:?}", regions)
```

So far implemented:
- [ ] Creating reports
- [ ] Deleting reports
- [ ] Getting reports
- [ ] Getting reports list
- [X] Getting regions list
