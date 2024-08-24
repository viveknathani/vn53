use vn53::resolver;

fn main() {
    resolver::Resolver::resolve("vivekn.dev", &"198.41.0.4").unwrap();
}
