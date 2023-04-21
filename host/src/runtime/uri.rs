use std::convert::Infallible;
use std::str::FromStr;

const SCHEME_SUFFIX: char = ':';
const AUTHORITY_PREFIX: &str = "//";
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

fn optional<'a>((head, _): (&'a str, &str)) -> Option<&'a str> {
    if head.is_empty() {
        None
    } else {
        Some(head)
    }
}

impl Uri {
    pub fn scheme(&self) -> Option<&str> {
        optional(self.scheme_split())
    }

    pub fn authority(&self) -> Option<Authority> {
        Some(Authority(optional(self.authority_split())?))
    }

    pub fn path(&self) -> Option<&str> {
        optional(self.path_split())
    }

    pub fn query(&self) -> Option<&str> {
        optional(self.query_split())
    }

    pub fn fragment(&self) -> Option<&str> {
        self.query_split().1.strip_prefix(FRAGMENT_PREFIX)
    }

    fn scheme_split(&self) -> (&str, &str) {
        match self.0.find(SCHEME_SUFFIX) {
            None => (self.empty(), &self.0),
            Some(index) => {
                let (scheme, rest) = self.0.split_at(index);

                (scheme, &rest[SCHEME_SUFFIX.len_utf8()..])
            }
        }
    }

    fn authority_split(&self) -> (&str, &str) {
        let scheme_rest = self.scheme_split().1;

        match scheme_rest.strip_prefix(AUTHORITY_PREFIX) {
            None => (self.empty(), scheme_rest),
            Some(rest) => match rest.find([PATH_PREFIX, QUERY_PREFIX, FRAGMENT_PREFIX]) {
                None => (rest, self.empty()),
                Some(index) => (&rest[..index], &rest[index..]),
            },
        }
    }

    fn path_split(&self) -> (&str, &str) {
        let rest = self.authority_split().1;

        if rest.starts_with(PATH_PREFIX) {
            match rest.find([QUERY_PREFIX, FRAGMENT_PREFIX]) {
                None => (rest, self.empty()),
                Some(index) => (&rest[..index], &rest[index..]),
            }
        } else {
            (self.empty(), rest)
        }
    }

    fn query_split(&self) -> (&str, &str) {
        let scheme_rest = self.path_split().1;

        match scheme_rest.strip_prefix(QUERY_PREFIX) {
            None => (self.empty(), scheme_rest),
            Some(rest) => match rest.find([FRAGMENT_PREFIX]) {
                None => (rest, self.empty()),
                Some(index) => (&rest[..index], &rest[index..]),
            },
        }
    }

    fn empty(&self) -> &str {
        &self.0[0..0]
    }
}

impl<'a> Authority<'a> {
    pub fn user_info(&self) -> Option<UserInfo> {
        Some(UserInfo(optional(self.user_info_split())?))
    }

    pub fn host(&self) -> Option<&str> {
        optional(self.host_split())
    }

    pub fn port(&self) -> Option<&str> {
        self.host_split().1.strip_prefix(PORT_PREFIX)
    }

    fn user_info_split(&self) -> (&str, &str) {
        match self.0.find(USER_INFO_SUFFIX) {
            None => (self.empty(), (self.0)),
            Some(index) => {
                let (user_info, rest) = self.0.split_at(index);

                (user_info, &rest[SCHEME_SUFFIX.len_utf8()..])
            }
        }
    }

    fn host_split(&self) -> (&str, &str) {
        let rest = self.user_info_split().1;

        match rest.find(PORT_PREFIX) {
            None => (rest, self.empty()),
            Some(index) => (&rest[..index], &rest[index..]),
        }
    }

    fn empty(&self) -> &str {
        &self.0[0..0]
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
