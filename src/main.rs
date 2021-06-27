use tankersdk::{Options, Core};

#[link(name = "tanker", modifiers = "+whole-archive")]
extern "C" {
    pub fn tanker_status(tanker: *mut u8) -> u8;
}

fn main() {
    println!("test1");
   if let Ok(mut runtime) = tokio::runtime::Runtime::new() {
        runtime.block_on(async move {
            let tanker = Core::new(Options::new("test".to_string(), "/tmp".to_owned())).await.unwrap();
            println!("{:?}", tanker);
        });
    }
    println!("test2");

    let _ = couchbase_lite::Database::open(&std::path::Path::new("."), couchbase_lite::DatabaseConfig::default());

    println!("test3");

    unsafe { tanker_status(std::ptr::null_mut()); }

}