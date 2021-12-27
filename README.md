# clia-local-time

A LocalTime implementation to set timezone manually.

Tracing-subscriber 3.x switched from `chrono` crate to `time`.
When using with LocalTime on some platforms it will cause to failure, when auto fetching the timezone from system.

So we build ourselves' LocalTime to fix it.
In this patch, you can pass the timezone manually to LocalTime, it default use UTC.

## Example

Cargo.toml:

```toml
[dependencies]
time = { version = "0.3", features = ["macros"] }
tracing-subscriber = { version = "0.3", features = ["fmt", "std", "time", "local-time"] }
clia-local-time = "0.2"
```

main.rs:

```rust
use time::macros::format_description;
use clia_local_time::LocalTime;

fn main {
    let timer = LocalTime::with_timezone(
        format_description!(
            "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]"
        ),
        (8, 0, 0),
    );
    tracing_subscriber::fmt()
        .with_timer(timer)
        .init();
}
```
