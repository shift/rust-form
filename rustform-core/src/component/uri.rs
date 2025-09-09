use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ComponentUri {
    pub scheme: UriScheme,
    pub path: String,
    pub version: Option<String>,
    pub subpath: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UriScheme {
    Registry, // rust-form/react-crud@^2.0.0
    Git,      // git+https://github.com/org/repo@v1.2.3
    GitHub,   // github:org/repo@v1.2.3
    GitLab,   // gitlab:org/repo@v1.2.3
    Path,     // path:./local/component
    File,     // file:///absolute/path/to/component
}

impl ComponentUri {
    pub fn new(scheme: UriScheme, path: String) -> Self {
        Self {
            scheme,
            path,
            version: None,
            subpath: None,
        }
    }

    pub fn with_version(mut self, version: String) -> Self {
        self.version = Some(version);
        self
    }

    pub fn with_subpath(mut self, subpath: String) -> Self {
        self.subpath = Some(subpath);
        self
    }

    pub fn resolve_url(&self) -> Result<String> {
        match &self.scheme {
            UriScheme::Registry => Ok(format!("https://registry.rust-form.dev/{}", self.path)),
            UriScheme::Git => {
                if self.path.starts_with("https://") || self.path.starts_with("http://") {
                    Ok(self.path.clone())
                } else {
                    Err(Error::ValidationError(format!(
                        "Invalid git URL: {}",
                        self.path
                    )))
                }
            }
            UriScheme::GitHub => {
                let parts: Vec<&str> = self.path.split('/').collect();
                if parts.len() != 2 {
                    return Err(Error::ValidationError(format!(
                        "GitHub URI must be in format 'org/repo': {}",
                        self.path
                    )));
                }
                Ok(format!("https://github.com/{}/{}", parts[0], parts[1]))
            }
            UriScheme::GitLab => {
                let parts: Vec<&str> = self.path.split('/').collect();
                if parts.len() != 2 {
                    return Err(Error::ValidationError(format!(
                        "GitLab URI must be in format 'org/repo': {}",
                        self.path
                    )));
                }
                Ok(format!("https://gitlab.com/{}/{}", parts[0], parts[1]))
            }
            UriScheme::Path | UriScheme::File => Ok(self.path.clone()),
        }
    }

    pub fn is_local(&self) -> bool {
        matches!(self.scheme, UriScheme::Path | UriScheme::File)
    }

    pub fn is_remote(&self) -> bool {
        !self.is_local()
    }

    pub fn cache_key(&self) -> String {
        match &self.version {
            Some(version) => format!("{}@{}", self, version),
            None => self.to_string(),
        }
    }
}

impl FromStr for ComponentUri {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (uri_part, subpath) = if let Some(pos) = s.find('#') {
            (&s[..pos], Some(s[pos + 1..].to_string()))
        } else {
            (s, None)
        };

        let (main_part, version) = if let Some(pos) = uri_part.rfind('@') {
            let potential_version = &uri_part[pos + 1..];
            if potential_version.contains('/') {
                (uri_part, None)
            } else {
                (&uri_part[..pos], Some(potential_version.to_string()))
            }
        } else {
            (uri_part, None)
        };

        let (scheme, path) = if let Some(colon_pos) = main_part.find(':') {
            let scheme_str = &main_part[..colon_pos];
            let path_str = &main_part[colon_pos + 1..];

            let scheme = match scheme_str {
                "git+https" | "git+http" => UriScheme::Git,
                "github" => UriScheme::GitHub,
                "gitlab" => UriScheme::GitLab,
                "path" => UriScheme::Path,
                "file" => UriScheme::File,
                _ => {
                    return Err(Error::ValidationError(format!(
                        "Unknown URI scheme: {}",
                        scheme_str
                    )))
                }
            };

            let cleaned_path = if scheme == UriScheme::Git && path_str.starts_with("//") {
                format!("https:{}", path_str)
            } else {
                path_str.to_string()
            };

            (scheme, cleaned_path)
        } else {
            (UriScheme::Registry, main_part.to_string())
        };

        let mut uri = ComponentUri::new(scheme, path);
        if let Some(v) = version {
            uri = uri.with_version(v);
        }
        if let Some(s) = subpath {
            uri = uri.with_subpath(s);
        }

        Ok(uri)
    }
}

impl fmt::Display for ComponentUri {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.scheme {
            UriScheme::Registry => write!(f, "{}", self.path)?,
            UriScheme::Git => {
                if self.path.starts_with("http") {
                    write!(f, "git+{}", self.path)?;
                } else {
                    write!(f, "git:{}", self.path)?;
                }
            }
            UriScheme::GitHub => write!(f, "github:{}", self.path)?,
            UriScheme::GitLab => write!(f, "gitlab:{}", self.path)?,
            UriScheme::Path => write!(f, "path:{}", self.path)?,
            UriScheme::File => write!(f, "file:{}", self.path)?,
        }

        if let Some(ref version) = self.version {
            write!(f, "@{}", version)?;
        }

        if let Some(ref subpath) = self.subpath {
            write!(f, "#{}", subpath)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_registry_uri() {
        let uri: ComponentUri = "rust-form/react-crud@^2.0.0".parse().unwrap();
        assert_eq!(uri.scheme, UriScheme::Registry);
        assert_eq!(uri.path, "rust-form/react-crud");
        assert_eq!(uri.version, Some("^2.0.0".to_string()));
    }

    #[test]
    fn test_parse_github_uri() {
        let uri: ComponentUri = "github:org/repo@v1.2.3".parse().unwrap();
        assert_eq!(uri.scheme, UriScheme::GitHub);
        assert_eq!(uri.path, "org/repo");
        assert_eq!(uri.version, Some("v1.2.3".to_string()));
    }

    #[test]
    fn test_parse_path_uri() {
        let uri: ComponentUri = "path:./local/component#subdir".parse().unwrap();
        assert_eq!(uri.scheme, UriScheme::Path);
        assert_eq!(uri.path, "./local/component");
        assert_eq!(uri.subpath, Some("subdir".to_string()));
    }

    #[test]
    fn test_parse_git_uri() {
        let uri: ComponentUri = "git+https://github.com/org/repo@main".parse().unwrap();
        assert_eq!(uri.scheme, UriScheme::Git);
        assert_eq!(uri.path, "https://github.com/org/repo");
        assert_eq!(uri.version, Some("main".to_string()));
    }

    #[test]
    fn test_display_uri() {
        let uri = ComponentUri::new(UriScheme::GitHub, "org/repo".to_string())
            .with_version("v1.0.0".to_string())
            .with_subpath("components".to_string());

        assert_eq!(uri.to_string(), "github:org/repo@v1.0.0#components");
    }
}
