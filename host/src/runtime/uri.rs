use std::convert::Infallible;
use std::str::FromStr;

const SCHEME_SUFFIX: char = ':';
const AUTHORITY_PREFIX: &'static str = "://";
const USER_INFO_SUFFIX: char = '@';
const PORT_PREFIX: char = ':';
const PATH_PREFIX: char = '/';
const QUERY_PREFIX: char = '?';
const FRAGMENT_PREFIX: char = '#';

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Uri(String);

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Authority<'a>(&'a str);

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct UserInfo<'a>(&'a str);

impl FromStr for Uri {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Uri::from(s))
    }
}

impl From<String> for Uri {
    fn from(value: String) -> Self {
        Uri(value)
    }
}

impl From<&str> for Uri {
    fn from(value: &str) -> Self {
        Uri(value.to_string())
    }
}

impl AsRef<str> for Uri {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl Uri {
    pub fn scheme(&self) -> Option<&str> {
        let index = self.scheme_location()?;

        Some(self.0.split_at(index).0)
    }

    pub fn authority(&self) -> Option<Authority> {
        let start_index = self.authority_location()? + AUTHORITY_PREFIX.len();
        let rest = &self.0[start_index..];

        match rest.find(&[PATH_PREFIX, QUERY_PREFIX, FRAGMENT_PREFIX]) {
            None => Some(Authority(rest)),
            Some(end_index) => Some(Authority(&rest[..end_index])),
        }
    }

    pub fn path(&self) -> Option<&str> {
        let rest_start_index = self.authority_location()? + AUTHORITY_PREFIX.len();
        let rest = &self.0[rest_start_index..];

        let start_index = rest.find(PATH_PREFIX)?;

        match rest.find(&[QUERY_PREFIX, FRAGMENT_PREFIX]) {
            None => Some(&rest[start_index..]),
            Some(end_index) => Some(&rest[start_index..end_index]),
        }
    }

    pub fn query(&self) -> Option<&str> {
        let start_index = self.query_location()? + QUERY_PREFIX.len_utf8();

        match self.fragment_location() {
            None => Some(&self.0[start_index..]),
            Some(end_index) => Some(&self.0[start_index..end_index]),
        }
    }

    pub fn fragment(&self) -> Option<&str> {
        let index = self.fragment_location()? + FRAGMENT_PREFIX.len_utf8();

        Some(self.0.split_at(index).1)
    }

    fn scheme_location(&self) -> Option<usize> {
        self.0.find(SCHEME_SUFFIX)
    }

    fn authority_location(&self) -> Option<usize> {
        self.0.find(AUTHORITY_PREFIX)
    }

    fn query_location(&self) -> Option<usize> {
        self.0.find(QUERY_PREFIX)
    }

    fn fragment_location(&self) -> Option<usize> {
        self.0.find(FRAGMENT_PREFIX)
    }
}

impl<'a> Authority<'a> {
    pub fn user_info(&self) -> Option<UserInfo> {
        let index = self.user_info_location()?;

        Some(UserInfo(self.0.split_at(index).0))
    }

    pub fn host(&self) -> Option<&str> {
        let start_index = match self.user_info_location() {
            None => 0,
            Some(index) => index + USER_INFO_SUFFIX.len_utf8(),
        };

        match self.port_location() {
            None => Some(&self.0[start_index..]),
            Some(end_index) => Some(&self.0[start_index..end_index]),
        }
    }

    pub fn port(&self) -> Option<&str> {
        let index = self.port_location()? + PORT_PREFIX.len_utf8();

        Some(self.0.split_at(index).1)
    }

    fn user_info_location(&self) -> Option<usize> {
        self.0.find(USER_INFO_SUFFIX)
    }

    fn port_location(&self) -> Option<usize> {
        self.0.rfind(PORT_PREFIX)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const URI: &str = "https://user:password@www.example.com:8443/index.html?query=value#fragment";

    #[test]
    fn scheme() {
        let uri = Uri::from(URI);

        assert_eq!(uri.scheme(), Some("https"))
    }

    #[test]
    fn authority() {
        let uri = Uri::from(URI);

        assert_eq!(
            uri.authority(),
            Some(Authority("user:password@www.example.com:8443"))
        );
    }

    #[test]
    fn user_info() {
        let uri = Uri::from(URI);

        assert_eq!(
            uri.authority().unwrap().user_info(),
            Some(UserInfo("user:password"))
        );
    }

    #[test]
    fn host() {
        let uri = Uri::from(URI);

        assert_eq!(uri.authority().unwrap().host(), Some("www.example.com"));
    }

    #[test]
    fn port() {
        let uri = Uri::from(URI);

        assert_eq!(uri.authority().unwrap().port(), Some("8443"));
    }

    #[test]
    fn path() {
        let uri = Uri::from(URI);

        assert_eq!(uri.path(), Some("/index.html"))
    }

    #[test]
    fn query() {
        let uri = Uri::from(URI);

        assert_eq!(uri.query(), Some("query=value"))
    }

    #[test]
    fn fragment() {
        let uri = Uri::from(URI);

        assert_eq!(uri.fragment(), Some("fragment"))
    }
}
