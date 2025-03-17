use logger::{Log, make_error, make_fatal};
use serde::{Deserialize, Serialize};
use serde_inline_default::serde_inline_default;
use std::path::PathBuf;
use tl::{
    Source,
    parser::parse,
    runtime::{Scope, types::Value},
};

#[serde_inline_default]
#[derive(Debug, Serialize, Deserialize)]
pub struct Package {
    /// String used to identify the package.
    /// Must be unique.
    pub id: String,
    /// Name of the package that will be displayed to user.
    /// Defaults to id.
    #[serde(default)]
    pub name: String,
    /// Version of the package.
    #[serde_inline_default("0.1.0".into())]
    pub version: String,
    /// Description of the package.
    #[serde_inline_default("No description".into())]
    pub description: String,
    /// Author of the package.
    #[serde(default)]
    pub authors: Vec<String>,

    /// Dependencies required to build the package.
    #[serde(default)]
    pub build_deps: Vec<Dependency>,
    /// Dependencies required to run the binary built from the package.
    #[serde(default)]
    pub runtime_deps: Vec<Dependency>,

    /// Path to the source directory of the package inside the sandbox.
    pub src: PathBuf,
    /// List of files expected in the build output directory.
    #[serde(default)]
    pub expected_output: Vec<PathBuf>,

    /// The nushell script that will be ran for the build stage.
    pub build: String,
    /// The nushell script that will be ran for the install stage.
    pub install: String,
}

#[derive(Debug)]
pub struct Dependency {
    pub id: String,
    pub version: Option<String>,
}

impl Serialize for Dependency {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match &self.version {
            Some(version) if !version.is_empty() => serializer.serialize_str(&format!("{}@{}", self.id, version)),
            _ => serializer.serialize_str(&self.id),
        }
    }
}

impl<'de> Deserialize<'de> for Dependency {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let parts: Vec<&str> = s.split('@').collect();

        Ok(Dependency {
            id: parts[0].to_string(),
            version: parts.iter().skip(1).last().map(ToString::to_string),
        })
    }
}

impl Package {
    pub fn eval(source: impl Into<Source>) -> Result<Self, Box<Log>> {
        let source = source.into();
        let ast = parse(&source).map_err(|err| Log::from(*err))?;
        let mut runtime = Scope::new(source, ast);

        let evaluated: Option<Package> = match runtime.eval() {
            Ok(value) if value != Value::Null => Ok(Some(Deserialize::deserialize(value).map_err(|err| Box::new(make_fatal!("Could not deserialize value: {err}")))?)),
            Ok(_) => Ok(None),
            Err(err) => Err(Box::new(Log::from(*err))),
        }?;

        match evaluated {
            Some(mut package) => {
                // Set the `name` property to the `id` if it's not set.
                if package.name.is_empty() {
                    package.name = package.id.clone();
                }

                Ok(package)
            }
            None => Err(Box::new(make_error!("Package source did not evaluate to anything."))),
        }
    }
}
