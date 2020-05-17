trait Destination {
    fn send(message: &[u8]);
}

trait Source {
    fn read(location: u32);
}
