use url::form_urlencoded::Serializer;

pub struct QueryBuilder {
    serializer: Serializer<'static, String>,
}

impl Default for QueryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl QueryBuilder {
    pub fn new() -> Self {
        QueryBuilder {
            serializer: Serializer::new(String::new()),
        }
    }

    pub fn push_opt<T: ToString>(&mut self, key: &str, value: Option<T>) {
        if let Some(value) = value {
            self.serializer.append_pair(key, &value.to_string());
        }
    }

    pub fn finish(mut self) -> String {
        self.serializer.finish()
    }
}
