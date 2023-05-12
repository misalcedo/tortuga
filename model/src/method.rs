use serde::{Deserialize, Serialize};

/// HTTP defines a set of request methods to indicate the desired action to be performed for a given resource.
/// Although they can also be nouns, these request methods are sometimes referred to as HTTP verbs.
/// Each of them implements a different semantic, but some common features are shared by a group of them:
/// e.g. a request method can be safe, idempotent, or cacheable.
///
/// See https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods
#[derive(
    Serialize, Deserialize, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash,
)]
#[repr(u8)]
pub enum Method {
    /// The `GET` method requests a representation of the specified resource. Requests using GET should only retrieve data.
    #[default]
    Get = 0,
    /// The `HEAD` method asks for a response identical to a `GET` request, but without the response body.
    Head = 1,
    /// The `POST` method submits an entity to the specified resource, often causing a change in state or side effects on the server.
    Post = 2,
    /// The `PUT` method replaces all current representations of the target resource with the request payload.
    Put = 3,
    /// The `DELETE` method deletes the specified resource.
    Delete = 4,
    /// The `CONNECT` method establishes a tunnel to the server identified by the target resource.
    Connect = 5,
    /// The `OPTIONS` method describes the communication options for the target resource.
    Options = 6,
    /// The `TRACE` method performs a message loop-back test along the path to the target resource.
    Trace = 7,
    /// The `PATCH` method applies partial modifications to a resource.
    Patch = 8,
}

impl From<Method> for u8 {
    fn from(method: Method) -> Self {
        method as u8
    }
}

impl TryFrom<u8> for Method {
    type Error = u8;

    fn try_from(method: u8) -> Result<Self, Self::Error> {
        match method {
            0 => Ok(Method::Get),
            1 => Ok(Method::Head),
            2 => Ok(Method::Post),
            3 => Ok(Method::Put),
            4 => Ok(Method::Delete),
            5 => Ok(Method::Connect),
            6 => Ok(Method::Options),
            7 => Ok(Method::Trace),
            8 => Ok(Method::Patch),
            _ => Err(method),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get() {
        assert_eq!(0.try_into(), Ok(Method::Get));
        assert_eq!(u8::from(Method::Get), 0);
    }

    #[test]
    fn head() {
        assert_eq!(1.try_into(), Ok(Method::Head));
        assert_eq!(u8::from(Method::Head), 1);
    }

    #[test]
    fn post() {
        assert_eq!(2.try_into(), Ok(Method::Post));
        assert_eq!(u8::from(Method::Post), 2);
    }

    #[test]
    fn put() {
        assert_eq!(3.try_into(), Ok(Method::Put));
        assert_eq!(u8::from(Method::Put), 3);
    }

    #[test]
    fn delete() {
        assert_eq!(4.try_into(), Ok(Method::Delete));
        assert_eq!(u8::from(Method::Delete), 4);
    }

    #[test]
    fn connect() {
        assert_eq!(5.try_into(), Ok(Method::Connect));
        assert_eq!(u8::from(Method::Connect), 5);
    }

    #[test]
    fn options() {
        assert_eq!(6.try_into(), Ok(Method::Options));
        assert_eq!(u8::from(Method::Options), 6);
    }

    #[test]
    fn trace() {
        assert_eq!(7.try_into(), Ok(Method::Trace));
        assert_eq!(u8::from(Method::Trace), 7);
    }

    #[test]
    fn patch() {
        assert_eq!(8.try_into(), Ok(Method::Patch));
        assert_eq!(u8::from(Method::Patch), 8);
    }

    #[test]
    fn unknown() {
        assert_eq!(Method::try_from(99u8), Err(99));
    }
}
