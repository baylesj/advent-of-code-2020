pub trait LoadableFromFile {
    fn load(filename: &str) -> Self;
}
