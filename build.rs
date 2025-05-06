fn main() {
    let sync = std::env::var("CARGO_FEATURE_SYNC").is_ok();
    let async_ = std::env::var("CARGO_FEATURE_ASYNC").is_ok();

    if sync && async_ {
        panic!("Cannot enable both `sync` and `async` features at the same time!");
    }
}
