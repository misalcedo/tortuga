use crate::runtime::Guest;
use crate::runtime::Uri;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tortuga_guest::Method;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Route {
    method: Option<Method>,
    uri: Uri,
}

impl<U: Into<Uri>> From<U> for Route {
    fn from(value: U) -> Self {
        Route {
            method: None,
            uri: value.into(),
        }
    }
}

impl Route {
    pub fn new(method: impl Into<Method>, uri: impl Into<Uri>) -> Self {
        Route {
            method: Some(method.into()),
            uri: uri.into(),
        }
    }
}

// TODO: Routing is done in the following order: Scheme -> Authority -> Path -> Method
#[derive(Clone, Debug, Default)]
pub struct Router {
    routes: Arc<RwLock<HashMap<Route, Guest>>>,
}

// TODO: Add support for any method.
// TODO: Add support for prefix match.
impl Router {
    pub fn define(
        &mut self,
        method: impl Into<Method>,
        uri: impl Into<Uri>,
        guest: &Guest,
    ) -> Option<Guest> {
        let mut routes = match self.routes.write() {
            Ok(routes) => routes,
            Err(mut e) => {
                e.get_mut().clear();
                e.into_inner()
            }
        };
        let route = Route::new(method, uri);

        routes.insert(route, guest.clone())
    }

    pub fn route(&self, method: impl Into<Method>, uri: impl Into<Uri>) -> Option<Guest> {
        let routes = self.routes.read().ok()?;
        let route = Route::new(method, uri);

        routes.get(&route).cloned()
    }
}
