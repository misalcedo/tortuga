pub use encoding::Encoding;
pub use message::Message;
pub use method::Method;
pub use request::Request;
pub use response::Response;
pub use status::Status;
pub use uri::Uri;
pub use wire::Wire;

mod encoding;
mod frame;
mod message;
mod method;
mod request;
mod response;
mod status;
mod uri;
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
