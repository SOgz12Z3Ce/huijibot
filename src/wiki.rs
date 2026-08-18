use std::{fmt::Display, path::Path};

#[derive(Clone)]
pub(crate) struct Title {
    namespace: Option<String>,
    name: String,
}

impl Title {
    pub(crate) fn new<P: AsRef<Path>>(file: P) -> Self {
        let path = file.as_ref();
        let name = path.with_extension("").to_str().unwrap().replace("\\", "/");
        let extension = path.extension().unwrap().to_str().unwrap();
        match extension {
            "lua" => Self {
                namespace: Some("Module".to_owned()),
                name: name,
            },
            "wikitext" => Self {
                namespace: None,
                name: name,
            },
            _ => todo!(),
        }
    }
}

impl Display for Title {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.namespace {
            Some(namespace) => write!(f, "{namespace}:{}", self.name),
            None => write!(f, "{}", self.name),
        }
    }
}
