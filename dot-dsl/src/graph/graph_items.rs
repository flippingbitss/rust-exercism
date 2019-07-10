

pub mod node {
    use std::collections::HashMap;

    #[derive(Debug, PartialEq, Clone)]
    pub struct Node {
        pub id: String,
        pub attrs: HashMap<String, String>,
    }

    impl Node {
        pub fn new(id: &str) -> Self {
            Node { id: id.to_string(), attrs: HashMap::new() }
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            self.attrs = attrs.iter().map(|x| (x.0.into(), x.1.into())).collect();
            self
        }

        pub fn get_attr(&self, id: &str) -> Option<&str> {
            self.attrs.get(id).map(String::as_str)
        }
    }
}

pub mod edge {
    use std::collections::HashMap;

    #[derive(Debug, PartialEq, Clone)]
    pub struct Edge {
        pub from: String,
        pub to: String,
        pub attrs: HashMap<String, String>,
    }

    impl Edge {
        pub fn new(from: &str, to: &str) -> Self {
            Edge { from: from.to_string(), to: to.to_string(), attrs: HashMap::new() }
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            self.attrs = attrs.iter().map(|x| (x.0.into(), x.1.into())).collect();
            self
        }

        pub fn get_attr(&self, id: &str) -> Option<&str> {
            self.attrs.get(id).map(String::as_str)
        }
    }
}
