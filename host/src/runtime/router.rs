use crate::runtime::Guest;
use crate::runtime::Uri;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tortuga_guest::Method;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Route {
    method: Method,
    uri: Uri,
}

impl Route {
    pub fn new(method: impl Into<Method>, uri: impl Into<Uri>) -> Self {
        Route {
            method: method.into(),
            uri: uri.into(),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Router {
    routes: Arc<RwLock<HashMap<Route, Guest>>>,
}

impl Router {
    pub fn define(
        &mut self,
        method: impl Into<Method>,
        uri: impl Into<Uri>,
        guest: Guest,
    ) -> Option<Guest> {
        let mut routes = match self.routes.write() {
            Ok(routes) => routes,
            Err(mut e) => {
                e.get_mut().clear();
                e.into_inner()
            }
        };
        let route = Route::new(method, uri);

        routes.insert(route, guest)
    }

    pub fn route(&self, method: impl Into<Method>, uri: impl Into<Uri>) -> Option<Guest> {
        let routes = self.routes.read().ok()?;
        let route = Route::new(method, uri);

        routes.get(&route).cloned()
    }
}
