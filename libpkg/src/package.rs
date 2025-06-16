use prelude::logger::{make_error, make_fatal, Log};
use serde::{Deserialize, Serialize};
use serde_inline_default::serde_inline_default;
use std::path::PathBuf;
use tl::{
    object,
    parser::parse,
    runtime::{
        types::{NativeFunction, Value},
        Scope,
    },
    Source,
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
    pub src: Src,
    /// List of files expected in the build output directory.
    #[serde(default)]
    pub expected_output: Vec<PathBuf>,

    /// The nushell script that will be ran for the build stage.
    pub build: String,
    /// The nushell script that will be ran for the install stage.
    pub install: String,

    /// Path to the package file.
    #[serde(skip)]
    pub(crate) path: Option<PathBuf>,
}

#[derive(Debug)]
pub enum Src {
    Path(PathBuf),
    // TODO: Use a proper URL type
    Git(String),
}

impl Serialize for Src {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Src::Path(path) => serializer.serialize_str(&path.display().to_string()),
            Src::Git(url) => serializer.serialize_str(url),
        }
    }
}

impl<'de> Deserialize<'de> for Src {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let string = String::deserialize(deserializer)?;
        let path = PathBuf::from(&string);

        // TODO: Once more source types are implemented this will need to be a bit smarter
        if path.exists() {
            Ok(Self::Path(path))
        } else {
            Ok(Self::Git(string))
        }
    }
}

#[derive(Debug)]
pub struct Dependency {
    pub id: String,
    pub version: Option<String>,
}

impl Serialize for Dependency {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match &self.version {
            Some(version) if !version.is_empty() => serializer.serialize_str(&format!("{}@{}", self.id, version)),
            _ => serializer.serialize_str(&self.id),
        }
    }
}

impl<'de> Deserialize<'de> for Dependency {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        let parts: Vec<&str> = s.split('@').collect();

        Ok(Self {
            id: parts[0].to_string(),
            version: parts.iter().skip(1).last().map(ToString::to_string),
        })
    }
}

impl Package {
    pub fn eval(source: impl Into<Source>) -> Result<Self, Box<Log>> {
        let source = source.into();
        let package_path = source.path.clone().map(|p| p.canonicalize().unwrap_or(p));
        let ast = parse(&source).map_err(|err| Log::from(*err))?;
        let mut scope = Scope::new(source, ast);

        scope.add_native_fn(
            "package",
            NativeFunction::Strict {
                params: 1,
                func: Box::new(|args| {
                    let Some(data @ Value::Object(_)) = args.first() else {
                        return Err(Box::new(tl::Error::new(
                            tl::runtime::ErrorType::NativeFnError("The `package` function requires an object as input".into()),
                            None,
                        )));
                    };

                    Ok(object!(kind = Value::String("Package".into()), data = data.clone()))
                }),
            },
        );

        scope.add_native_fn(
            "group",
            NativeFunction::Strict {
                params: 1,
                func: Box::new(|args| {
                    let Some(data @ Value::Object(_)) = args.first() else {
                        return Err(Box::new(tl::Error::new(
                            tl::runtime::ErrorType::NativeFnError("The `group` function requires an object as input".into()),
                            None,
                        )));
                    };

                    Ok(object!(kind = Value::String("Group".into()), data = data.clone()))
                }),
            },
        );

        let evaluated: Option<Package> = match scope.eval() {
            Ok(value) if value != Value::Null => {
                dbg!(&value);
                Ok(Some(Deserialize::deserialize(value).map_err(|err| Box::new(make_fatal!("Could not deserialize value: {err}")))?))
            }
            Ok(_) => Ok(None),
            Err(err) => Err(Box::new(Log::from(*err))),
        }?;

        match evaluated {
            Some(mut package) => {
                // Set the `name` property to the `id` if it's not set.
                if package.name.is_empty() {
                    package.name = package.id.clone();
                }

                package.path = package_path;

                Ok(package)
            }
            None => Err(Box::new(make_error!("Package source did not evaluate to anything."))),
        }
    }
}
