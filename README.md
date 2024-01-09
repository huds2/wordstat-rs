# wordstat-rs 

Rust library for interacting with the Yandex Wordstat API

## Usage:

Create the client:
```rust
let client = Client::new("token", "https://api-sandbox.direct.yandex.ru/v4/json/");
```
Get the list of available regions:
```rust
let regions = get_regions(&client).await.unwrap();
```
Start the report generation:
```rust
let request = ReportRequest::new()
    .add_phrase("rust")
    .add_geo(54); // Geo is optional
let report_id = create_report(&client, &request).await.unwrap();
```
Getting the list of all available reports and their statuses:
```rust
let report_list = get_report_list(&client).awai.unwrap()t;
```
Get the generated report:
```rust
let report = get_reports(&client, report_id).await.unwrap();
```
Delete the reports (you can have more than 5 reports on your account simultaneously):
```rust
delete_report(&client, report_id).await.unwrap();
```

Stuff to do:
- [X] Creating reports
- [X] Deleting reports
- [X] Getting reports
- [X] Getting reports list
- [X] Getting regions list
- [ ] Cover the methods with tests
- [ ] Documentation
