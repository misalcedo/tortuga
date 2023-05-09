pub use encoding::Encoding;

mod encoding;
mod frame;
mod message;
mod method;
mod request;
mod response;
mod status;
mod wire;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
