use tankersdk::{Options, Core};

fn main() {
    //let _ = couchbase_lite::Database::open(&std::path::Path::new("."), couchbase_lite::DatabaseConfig::default());

    if let Ok(mut runtime) = tokio::runtime::Runtime::new() {
        runtime.block_on(async move {
            let tanker = Core::new(Options::new("test".to_string(), "/tmp".to_owned())).await.unwrap();
            println!("{:?}", tanker);
        });
    }
}
