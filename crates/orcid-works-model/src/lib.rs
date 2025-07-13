use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Value<T> {
    #[serde(rename = "value")]
    pub value: T,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExternalId {
    #[serde(rename = "external-id-type")]
    pub external_id_type: String,
    #[serde(rename = "external-id-value")]
    pub external_id_value: String,
    #[serde(rename = "external-id-url", skip_serializing_if = "Option::is_none")]
    pub external_id_url: Option<Value<String>>,
    #[serde(rename = "external-id-relationship")]
    pub external_id_relationship: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceRef {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Source {
    #[serde(rename = "source-orcid", skip_serializing_if = "Option::is_none")]
    pub source_orcid: Option<SourceRef>,
    #[serde(rename = "source-client-id", skip_serializing_if = "Option::is_none")]
    pub source_client_id: Option<SourceRef>,
    #[serde(rename = "source-name", skip_serializing_if = "Option::is_none")]
    pub source_name: Option<Value<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Title {
    pub title: Value<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<Value<String>>,
    #[serde(rename = "translated-title", skip_serializing_if = "Option::is_none")]
    pub translated_title: Option<TranslatedTitle>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TranslatedTitle {
    pub value: String,
    #[serde(rename = "language-code")]
    pub language_code: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicationDate {
    pub year: Value<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month: Option<Value<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day: Option<Value<String>>,
    #[serde(rename = "media-type", skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Contributor {
    #[serde(rename = "contributor-orcid", skip_serializing_if = "Option::is_none")]
    pub contributor_orcid: Option<SourceRef>,
    #[serde(rename = "credit-name", skip_serializing_if = "Option::is_none")]
    pub credit_name: Option<Value<String>>,
    #[serde(rename = "contributor-email", skip_serializing_if = "Option::is_none")]
    pub contributor_email: Option<Value<String>>,
    #[serde(
        rename = "contributor-attributes",
        skip_serializing_if = "Option::is_none"
    )]
    pub contributor_attributes: Option<ContributorAttributes>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContributorAttributes {
    #[serde(
        rename = "contributor-sequence",
        skip_serializing_if = "Option::is_none"
    )]
    pub contributor_sequence: Option<String>,

    #[serde(rename = "contributor-role", skip_serializing_if = "Option::is_none")]
    pub contributor_role: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExternalIds {
    #[serde(rename = "external-id", skip_serializing_if = "Option::is_none")]
    pub external_id: Option<Vec<ExternalId>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrcidWorkSummary {
    #[serde(rename = "put-code")]
    pub put_code: u64,
    #[serde(rename = "created-date")]
    pub created_date: Value<u64>,
    #[serde(rename = "last-modified-date")]
    pub last_modified_date: Value<u64>,
    pub source: Source,
    pub title: Title,
    #[serde(rename = "external-ids")]
    pub external_ids: ExternalIds,
    pub r#type: String,
    #[serde(rename = "publication-date", skip_serializing_if = "Option::is_none")]
    pub publication_date: Option<PublicationDate>,
    pub visibility: String,
    pub path: String,
    #[serde(rename = "display-index", skip_serializing_if = "Option::is_none")]
    pub display_index: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Citation {
    #[serde(rename = "citation-type")]
    pub citation_type: String,
    #[serde(rename = "citation-value")]
    pub citation_value: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Contributors {
    #[serde(rename = "contributor", skip_serializing_if = "Option::is_none")]
    pub contributor: Option<Vec<Contributor>>,
}

// Response from /{id}/work/{putcode}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrcidWorkDetail {
    #[serde(flatten)]
    pub summary: OrcidWorkSummary,

    #[serde(rename = "journal-title", skip_serializing_if = "Option::is_none")]
    pub journal_title: Option<Value<String>>,
    #[serde(rename = "short-description", skip_serializing_if = "Option::is_none")]
    pub short_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub citation: Option<Citation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Value<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contributors: Option<Contributors>,
    #[serde(rename = "language-code", skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Value<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkGroup {
    #[serde(rename = "last-modified-date")]
    pub last_modified_date: Value<u64>,
    #[serde(rename = "external-ids")]
    pub external_ids: ExternalIds,
    #[serde(rename = "work-summary")]
    pub work_summary: Vec<OrcidWorkSummary>,
}

// Response JSON from {BASE}/{id}/works
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrcidWorks {
    #[serde(rename = "last-modified-date")]
    pub last_modified_date: Value<u64>,
    pub group: Vec<WorkGroup>,
    pub path: String,
}

impl OrcidWorks {
    pub fn from_reader<R: std::io::Read>(reader: R) -> Result<Self, serde_json::Error> {
        serde_json::from_reader(reader)
    }
}

// On‑disk JSON wrapper: `{ "records": [ ... ] }`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrcidWorkDetailFile {
    pub records: Vec<OrcidWorkDetail>,
}

impl OrcidWorkDetail {
    pub fn from_reader<R: std::io::Read>(reader: R) -> Result<Self, serde_json::Error> {
        serde_json::from_reader(reader)
    }
}

impl OrcidWorkDetailFile {
    pub fn from_reader<R: std::io::Read>(reader: R) -> Result<Self, serde_json::Error> {
        serde_json::from_reader(reader)
    }
}
