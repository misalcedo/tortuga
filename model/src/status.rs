use serde::{Deserialize, Serialize};

/// See <https://developer.mozilla.org/en-US/docs/Web/HTTP/Status>
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash)]
pub enum Status {
    Continue,
    Ok,
    Created,
    MultipleChoices,
    BadRequest,
    Unauthorized,
    PaymentRequired,
    Forbidden,
    NotFound,
    MethodNotAllowed,
    NotAcceptable,
    ProxyAuthenticationRequired,
    RequestTimeout,
    Conflict,
    Gone,
    LengthRequired,
    PreconditionFailed,
    PayloadTooLarge,
    URITooLong,
    InternalServerError,
    Custom(u16),
}

impl PartialEq<u16> for Status {
    fn eq(&self, other: &u16) -> bool {
        u16::from(*self) == *other
    }
}

impl From<Status> for u16 {
    fn from(status: Status) -> Self {
        match status {
            Status::Continue => 100,
            Status::Ok => 200,
            Status::Created => 201,
            Status::MultipleChoices => 300,
            Status::BadRequest => 400,
            Status::Unauthorized => 401,
            Status::PaymentRequired => 402,
            Status::Forbidden => 403,
            Status::NotFound => 404,
            Status::MethodNotAllowed => 405,
            Status::NotAcceptable => 406,
            Status::ProxyAuthenticationRequired => 407,
            Status::RequestTimeout => 408,
            Status::Conflict => 409,
            Status::Gone => 410,
            Status::LengthRequired => 411,
            Status::PreconditionFailed => 412,
            Status::PayloadTooLarge => 413,
            Status::URITooLong => 414,
            Status::InternalServerError => 500,
            Status::Custom(s) => s,
        }
    }
}

impl From<u16> for Status {
    fn from(status: u16) -> Self {
        match status {
            100..=199 => Status::Continue,
            200..=299 => match status {
                200 => Status::Ok,
                201 => Status::Created,
                _ => Status::Custom(status),
            },
            300..=399 => Status::MultipleChoices,
            400..=499 => Status::BadRequest,
            500..=599 => Status::InternalServerError,
            _ => Status::Custom(status),
        }
    }
}
