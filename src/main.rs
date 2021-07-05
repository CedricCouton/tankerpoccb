use tankersdk::{Options, Core};

fn main() {
    let _tanker = Core::new(Options::new("test".to_string(), "/tmp".to_owned()));
}
