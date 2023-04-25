use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tortuga_guest::Uri;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Route {
    uri: Uri,
}

impl From<Uri> for Route {
    fn from(uri: Uri) -> Self {
        Route { uri }
    }
}

// TODO: Routing is done in the following order: Scheme -> Authority -> Path -> Method
#[derive(Clone, Debug, Default)]
pub struct Router<Target> {
    routes: Arc<RwLock<HashMap<Route, Target>>>,
}

// TODO: Add support for any method.
// TODO: Add support for prefix match.
impl<Target> Router<Target>
where
    Target: Clone,
{
    pub fn define(&mut self, uri: Uri, target: Target) -> Option<Target> {
        let mut routes = match self.routes.write() {
            Ok(routes) => routes,
            Err(e) => e.into_inner(),
        };
        let route = Route::from(uri);

        routes.insert(route, target)
    }

    pub fn route(&self, uri: Uri) -> Option<Target> {
        let routes = match self.routes.read() {
            Ok(routes) => routes,
            Err(e) => e.into_inner(),
        };
        let route = Route::from(uri);

        routes.get(&route).cloned()
    }
}
