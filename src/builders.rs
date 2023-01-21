//soon

use std::collections::HashMap;

pub struct SearchQueryBuilder {
    query: String,
    filters: HashMap<String, Vec<String>>,
}

impl SearchQueryBuilder {
    fn new() -> Self {
        SearchQueryBuilder {
            query: String::new(),
            filters: Default::default(),
        }
    }

    fn build(self) -> String {
        let mut query = vec![];
        if self.query.len() != 0 {
            query.push(self.query);
        }
        for (filter, values) in self.filters {
            query.push(format!("{}:{}", filter, values.join(",")));
        }

        query.join(" ")
    }

    fn query(mut self, query: impl ToString) -> Self {
        self.query = query.to_string();

        self
    }

    fn port(mut self, filter: impl ToString) -> Self {
        let ports = self
            .filters
            .entry("port".into())
            .or_insert(Default::default());

        ports.push(filter.to_string());

        self
    }

    fn product(mut self, filter: impl ToString) -> Self {
        let ports = self
            .filters
            .entry("product".into())
            .or_insert(Default::default());

        ports.push(filter.to_string());

        self
    }

    fn sll(mut self, ) -> Self {

    }
}

#[cfg(test)]
pub mod tests {
    use crate::builders::SearchQueryBuilder;

    #[test]
    fn can_build_query() {
        let query = SearchQueryBuilder::new()
            .query("apache")
            .port("69")
            .port(420)
            .product("Apache")
            .product("Bingus")
            .build();

        assert!(query.contains("port:69,420")           , "query was: {}", query);
        assert!(query.contains("product:Apache,Bingus") , "query was: {}", query);
        assert!(query.starts_with("apache")             , "query was: {}", query);
    }

    #[test]
    fn can_build_without_query() {
        let query = SearchQueryBuilder::new()
            .port("69")
            .port(420)
            .product("Apache")
            .product("Bingus")
            .build();

        assert!(query.contains("port:69,420")           , "query was: {}", query);
        assert!(query.contains("product:Apache,Bingus") , "query was: {}", query);
    }
}
