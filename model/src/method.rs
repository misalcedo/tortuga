use serde::{Deserialize, Serialize};

/// HTTP defines a set of request methods to indicate the desired action to be performed for a given resource.
/// Although they can also be nouns, these request methods are sometimes referred to as HTTP verbs.
/// Each of them implements a different semantic, but some common features are shared by a group of them:
/// e.g. a request method can be safe, idempotent, or cacheable.
///
/// See https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash)]
pub enum Method {
    /// The `GET` method requests a representation of the specified resource. Requests using GET should only retrieve data.
    Get,
    /// The `HEAD` method asks for a response identical to a `GET` request, but without the response body.
    Head,
    /// The `POST` method submits an entity to the specified resource, often causing a change in state or side effects on the server.
    Post,
    /// The `PUT` method replaces all current representations of the target resource with the request payload.
    Put,
    /// The `DELETE` method deletes the specified resource.
    Delete,
    /// The `CONNECT` method establishes a tunnel to the server identified by the target resource.
    Connect,
    /// The `OPTIONS` method describes the communication options for the target resource.
    Options,
    /// The `TRACE` method performs a message loop-back test along the path to the target resource.
    Trace,
    /// The `PATCH` method applies partial modifications to a resource.
    Patch,
}
