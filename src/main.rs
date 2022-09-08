fn main() {
    for meta in inventory_issue::Meta::collect() {
        dbg!(meta);
    }
}
