fn main() {
    for meta in inner::Meta::collect() {
        dbg!(meta);
    }

    dbg!("test");
}
