pub use frame::Frame;
pub use message::Message;
pub use method::Method;
pub use request::Request;
pub use response::Response;
pub use status::Status;
pub use uri::Uri;

pub mod asynchronous;
pub mod frame;
mod message;
mod method;
mod request;
mod response;
mod status;
pub mod synchronous;
mod uri;

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
