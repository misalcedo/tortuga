pub trait ToMetaVariable {
    fn to_meta_variable(&self, prefix: Option<&str>) -> String;
}

impl ToMetaVariable for str {
    fn to_meta_variable(&self, prefix: Option<&str>) -> String {
        let capacity = self.len() + prefix.map(|p| p.len() + '_'.len_utf8()).unwrap_or(0);

        let mut variable = String::with_capacity(capacity);
        let iterator = prefix
            .iter()
            .flat_map(|p| p.chars().chain(Some('_')))
            .chain(self.chars());

        for c in iterator {
            match c {
                '-' => variable.push('_'),
                _ if c.is_lowercase() => c.to_uppercase().for_each(|u| variable.push(u)),
                _ => variable.push(c),
            }
        }

        variable
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn with_prefix() {
        assert_eq!(
            "content-length".to_meta_variable(Some("http")).as_str(),
            "HTTP_CONTENT_LENGTH"
        );
    }

    #[test]
    fn without_prefix() {
        assert_eq!("server name".to_meta_variable(None).as_str(), "SERVER NAME");
    }

    #[test]
    fn with_empty_prefix() {
        assert_eq!(
            "server_name".to_meta_variable(Some("")).as_str(),
            "_SERVER_NAME"
        );
    }
}
