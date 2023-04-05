/// See https://developer.mozilla.org/en-US/docs/Web/HTTP/Status
#[derive(Debug, Default, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
#[repr(u16)]
pub enum Status {
    NoResponse = 0,
    Continue = 100,
    #[default]
    Ok = 200,
    Created = 201,
    MultipleChoices = 300,
    BadRequest = 400,
    Unauthorized = 401,
    PaymentRequired = 402,
    Forbidden = 403,
    NotFound = 404,
    MethodNotAllowed = 405,
    NotAcceptable = 406,
    ProxyAuthenticationRequired = 407,
    RequestTimeout = 408,
    Conflict = 409,
    Gone = 410,
    LengthRequired = 411,
    PreconditionFailed = 412,
    PayloadTooLarge = 413,
    URITooLong = 414,
    InternalServerError = 500,
}

impl PartialEq<u16> for Status {
    fn eq(&self, other: &u16) -> bool {
        &(*self as u16) == other
    }
}

impl From<Status> for u16 {
    fn from(status: Status) -> Self {
        status as u16
    }
}

impl TryFrom<u16> for Status {
    type Error = u16;

    fn try_from(status: u16) -> Result<Self, Self::Error> {
        match status {
            0 => Ok(Status::NoResponse),
            100..=199 => Ok(Status::Continue),
            200..=299 => match status {
                200 => Ok(Status::Ok),
                201 => Ok(Status::Created),
                _ => Err(status),
            },
            300..=399 => Ok(Status::MultipleChoices),
            400..=499 => Ok(Status::BadRequest),
            500..=599 => Ok(Status::InternalServerError),
            _ => Err(status),
        }
    }
}
