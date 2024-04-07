use std::collections::HashMap;

#[derive(Default)]
pub struct SslFilterBuilder {
    filters: HashMap<String, Vec<String>>,
}

impl SslFilterBuilder {
    pub fn build(self) -> HashMap<String, Vec<String>> {
        self.filters
    }

    pub fn cert_subject_cn(mut self, value: impl ToString) -> Self {
        let values = self.filters.entry("cert.subject.cn".into()).or_default();

        values.push(value.to_string());

        self
    }
}

#[derive(Default)]
pub struct SearchQueryBuilder {
    query: String,
    filters: HashMap<String, Vec<String>>,
}

impl SearchQueryBuilder {
    pub fn build(self) -> String {
        let mut query = vec![];
        if !self.query.is_empty() {
            query.push(self.query);
        }

        for (filter, values) in self.filters {
            query.push(format!("{}:{}", filter, values.join(",")));
        }

        query.join(" ")
    }

    pub fn query(mut self, query: impl ToString) -> Self {
        self.query = query.to_string();

        self
    }

    pub fn port(mut self, value: impl ToString) -> Self {
        let values = self.filters.entry("port".into()).or_default();

        values.push(value.to_string());

        self
    }

    pub fn product(mut self, filter: impl ToString) -> Self {
        let values = self.filters.entry("product".into()).or_default();

        values.push(filter.to_string());

        self
    }

    pub fn ssl(mut self, closure: fn(SslFilterBuilder) -> SslFilterBuilder) -> Self {
        let filters = closure(SslFilterBuilder::default()).build();

        self.filters.extend(filters);

        self
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn can_build_query() {
        let query = SearchQueryBuilder::default()
            .query("apache")
            .port("69")
            .port(420)
            .product("Apache")
            .product("Ngnix")
            .build();

        assert!(query.contains("port:69,420"), "query was: {query}");
        assert!(query.contains("product:Apache,Ngnix"), "query was: {query}");
        assert!(query.starts_with("apache"), "query was: {query}");
    }

    #[test]
    fn can_build_without_query() {
        let query = SearchQueryBuilder::default()
            .port("69")
            .port(420)
            .product("Apache")
            .product("Ngnix")
            .build();

        assert!(query.contains("port:69,420"), "query was: {query}");
        assert!(query.contains("product:Apache,Ngnix"), "query was: {query}");
    }

    #[test]
    fn can_build_with_ssl() {
        let query = SearchQueryBuilder::default()
            .port("69")
            .port(420)
            .product("Apache")
            .product("Ngnix")
            .ssl(|ssl_builder| {
                ssl_builder
                    .cert_subject_cn("google.com")
                    .cert_subject_cn("bing.com")
            })
            .build();

        assert!(query.contains("port:69,420"), "query was: {query}");
        assert!(query.contains("product:Apache,Ngnix"), "query was: {query}");
        assert!(
            query.contains("cert.subject.cn:google.com,bing.com"),
            "query was: {query}"
        );
    }
}
