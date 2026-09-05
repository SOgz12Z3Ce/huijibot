use std::{convert::Infallible, str::FromStr};

#[derive(Clone, Copy)]
pub(crate) enum Namespace {
    Main,
    Template,
    Module,
    Data,
}

impl Namespace {
    pub(crate) fn prefix(&self) -> Option<&'static str> {
        match self {
            Namespace::Main => None,
            Namespace::Template => Some("Template"),
            Namespace::Module => Some("Module"),
            Namespace::Data => Some("Data"),
        }
    }

    pub(crate) fn suffix(&self) -> Option<&'static str> {
        match self {
            Namespace::Main => None,
            Namespace::Template => None,
            Namespace::Module => None,
            Namespace::Data => Some(".json"),
        }
    }
}

impl FromStr for Namespace {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "main" => Ok(Namespace::Main),
            "template" => Ok(Namespace::Template),
            "module" => Ok(Namespace::Module),
            "data" => Ok(Namespace::Data),
            _ => todo!(),
        }
    }
}
