use vn53::resolver::{self, DNS_RECORD_TYPE_A};

fn main() {
    resolver::Resolver::resolve("vivekn.dev", DNS_RECORD_TYPE_A).unwrap();
}
