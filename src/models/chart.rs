// 🐻‍❄️📦 charted_sdk: Rust SDK library for Noelware's Charts Platform
// Copyright (c) 2022-2023 Noelware, LLC. <team@noelware.org>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{de::Visitor, Deserialize, Serialize};

/// The `apiVersion` field in a **Chart.yaml** file. This should be set to v2 that require atleast Helm 3, charts supporting
/// previous Helm charts should use v1 and are still installable by Helm 3.
#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize)]
pub enum ApiVersion {
    /// Charts that support previous Helm versions and are also installable by Helm 3.
    #[serde(rename = "v1")]
    V1,

    /// Charts that support only Helm 3.
    #[serde(rename = "v2")]
    V2,
}

#[allow(clippy::derivable_impls)]
impl Default for ApiVersion {
    fn default() -> Self {
        ApiVersion::V2
    }
}

/// Represents the repository type that **charted-server** supports.
#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize)]
pub enum RepositoryType {
    #[serde(rename = "application")]
    Application,

    #[serde(rename = "library")]
    Library,

    #[serde(rename = "operator")]
    Operator,
}

#[allow(clippy::derivable_impls)]
impl Default for RepositoryType {
    fn default() -> Self {
        RepositoryType::Application
    }
}

#[derive(Debug, Clone)]
pub struct StringOrImportValue {
    string: Option<String>,
    import_value: Option<ImportValue>,
}

impl StringOrImportValue {
    pub fn string(&self) -> Option<String> {
        self.string.clone()
    }

    pub fn import_value(&self) -> Option<ImportValue> {
        self.import_value.clone()
    }
}

impl Serialize for StringOrImportValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if let Some(string) = self.string.clone() {
            return serializer.serialize_str(string.as_str());
        }

        let import_value = self.import_value.clone().expect("missing import-value");
        import_value.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for StringOrImportValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(StringOrImportValueVisitor)
    }
}

struct StringOrImportValueVisitor;
impl<'de> Visitor<'de> for StringOrImportValueVisitor {
    type Value = StringOrImportValue;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("String or ImportValue")
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(StringOrImportValue {
            string: Some(v),
            import_value: None,
        })
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(StringOrImportValue {
            string: Some(v.to_owned()),
            import_value: None,
        })
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut child = None;
        let mut parent = None;

        while let Some(key) = map.next_key()? {
            match key {
                "child" => {
                    if child.is_some() {
                        return Err(serde::de::Error::duplicate_field("child"));
                    }

                    child = Some(map.next_value()?);
                }

                "parent" => {
                    if parent.is_some() {
                        return Err(serde::de::Error::duplicate_field("parent"));
                    }

                    parent = Some(map.next_value()?);
                }

                _ => return Err(serde::de::Error::unknown_field(key, &["child", "parent"])),
            }
        }

        if parent.is_none() {
            return Err(serde::de::Error::missing_field("parent"));
        }

        if child.is_none() {
            return Err(serde::de::Error::missing_field("child"));
        }

        Ok(StringOrImportValue {
            string: None,
            import_value: Some(ImportValue {
                parent: parent.unwrap(),
                child: child.unwrap(),
            }),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub struct ImportValue {
    /// The source key of the values to be imported
    pub child: String,

    /// The destination path in the parent chart's values
    pub parent: String,
}

/// The index file is a yaml file called `index.yaml`. It contains some metadata about the package,
/// including the contents of a chart's `Chart.yaml` file. A valid chart repository must have an index file.
/// The index file contains information about each chart in the chart repository.
/// The `helm repo index` command will generate an index file based on a given local directory that contains packaged charts.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ChartIndexYaml {
    pub api_version: String,
    pub entries: HashMap<String, Vec<ChartIndexSpec>>,
    pub generated: DateTime<Utc>,
}

/// In Helm, one chart may depend on any number of other charts. These dependencies can be dynamically linked using the
/// dependencies' field in Chart.yaml or brought in to the charts/ directory and managed manually. The charts required by the
/// current chart are defined as a list in the dependencies field.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ChartDependency {
    /// The name of the chart
    pub name: String,

    /// The version of the chart.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    /// The repository URL or alias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,

    /// A YAML path that resolves to a [`bool`], used for enabling or
    /// disabling charts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,

    /// Tags that can be used to group charts together.
    pub tags: Vec<String>,

    /// Import values holds the mapping of source values to parent keys to be imported. Each
    /// item can be a string or pair of child/parent sublist items.
    #[serde(rename = "import-values")]
    pub import_values: Vec<StringOrImportValue>,

    /// Alias to be used for the chart. Useful when you have to add the same chart multiple times
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ChartMaintainer {
    /// The maintainer's name
    pub name: String,

    /// The maintainer's email address
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// The maintainer's website or URL towards to the maintainer of this chart.
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ChartIndexSpec {
    /// The apiVersion field should be v2 for Helm charts that require at least Helm 3. Charts supporting previous
    /// Helm versions have an apiVersion set to v1 and are still installable by Helm 3.
    pub api_version: ApiVersion,

    /// The name of the chart
    pub name: String,

    /// A valid SemVer 2 version key
    pub version: String,

    /// The optional kubeVersion field can define SemVer constraints on supported Kubernetes versions. Helm will validate the version constraints
    /// when installing the chart and fail if the cluster runs an unsupported Kubernetes version.
    #[serde(rename = "kubeVersion", skip_serializing_if = "Option::is_none")]
    pub kube_version: Option<String>,

    /// Single-sentence description of this Helm chart
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The type of the chart
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub chart_type: Option<RepositoryType>,

    /// A list of keywords about this project
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub keywords: Vec<String>,

    /// The URL of this Helm chart's homepage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home: Option<String>,

    /// A list of URLs to the source code for the Helm chart.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub sources: Vec<String>,

    /// In Helm, one chart may depend on any number of other charts. These dependencies can be dynamically linked using the dependencies'
    /// field in Chart.yaml or brought in to the charts/ directory and managed manually. The charts required by the current chart are defined as a list in
    /// the dependencies field.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub dependencies: Vec<ChartDependency>,

    /// List of maintainers that maintain this Helm chart
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub maintainers: Vec<ChartMaintainer>,

    /// A URL, SVG icon, or image to be used as the repository's icon.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,

    /// Note that the appVersion field is not related to the version field. It is a way of specifying the version of the
    /// application. For example, the drupal chart may have an appVersion: "8.2.1", indicating that the version of Drupal
    /// included in the chart (by default) is 8.2.1. This field is informational, and has no impact on chart version calculations.
    ///
    /// Wrapping the version in quotes is highly recommended. It forces the YAML parser to treat the version number as a string.
    /// Leaving it unquoted can lead to parsing issues in some cases. For example, YAML interprets 1.0 as a floating point value,
    /// and a git commit SHA like 1234e10 as scientific notation.
    #[serde(rename = "appVersion", skip_serializing_if = "Option::is_none")]
    pub app_version: Option<String>,

    /// When managing charts in a Chart Repository, it is sometimes necessary to deprecate a chart. The optional deprecated field
    /// in Chart.yaml can be used to mark a chart as deprecated. If the latest version of a chart in the repository is marked
    /// as deprecated, then the chart as a whole is considered to be deprecated.
    ///
    /// The chart name can be later reused by publishing a newer version that is not marked as deprecated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,

    /// List of annotations keyed by name and value.
    pub annotations: HashMap<String, String>,
}
