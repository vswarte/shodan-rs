pub struct FacetBuilder {
    facets: Vec<String>,
}

impl FacetBuilder {
    pub fn new() -> Self {
        FacetBuilder { facets: vec![] }
    }

    pub fn add(mut self,facet: &str) -> Self {
        self.facets.push(facet.into());
        self
    }

    pub fn build(self) -> String {
        self.facets.join(",")
    }
}

#[cfg(test)]
pub mod tests {
    use crate::builders::FacetBuilder;

    #[test]
    fn build_facet_string() {
        let facet_string = FacetBuilder::new().add("os").add("country").add("asn").build();

        assert_eq!(facet_string, "os,country,asn","facet string was {}", facet_string);
    }

    #[test]
    fn can_get_count_with_facets() {
        let facet_string = FacetBuilder::new().add("os").add("country").add("asn").build();

        assert_eq!(facet_string, "os,country,asn","facet string was {}", facet_string);
    }

}
