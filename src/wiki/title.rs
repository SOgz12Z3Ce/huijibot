use crate::wiki::Namespace;
use std::{fmt::Display, path::Path};

#[derive(Clone)]
pub(crate) struct Title {
    namespace: Namespace,
    name: String,
}

impl Title {
    pub(crate) fn new<P: AsRef<Path>>(file: P) -> Self {
        let path = file.as_ref();
        let extension = path.extension().unwrap().to_str().unwrap();
        let name = path.with_extension("").to_str().unwrap().replace("\\", "/");
        Self {
            namespace: match extension {
                "wikitext" => Namespace::Main,
                "lua" => Namespace::Module,
                "json" => Namespace::Data,
                _ => todo!(),
            },
            name,
        }
    }

    pub(crate) fn with_namespace(self, namespace: Namespace) -> Self {
        Self {
            namespace: namespace,
            ..self
        }
    }
}

impl Display for Title {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(prefix) = self.namespace.prefix() {
            write!(f, "{prefix}:")?;
        }
        write!(f, "{}", self.name)?;
        if let Some(suffix) = self.namespace.suffix() {
            write!(f, "{suffix}")?;
        }
        Ok(())
    }
}
