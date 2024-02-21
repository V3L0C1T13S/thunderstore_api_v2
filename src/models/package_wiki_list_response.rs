/*
 * Thunderstore API
 *
 * Schema is automatically generated and not completely accurate.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PackageWikiListResponse {
    #[serde(rename = "results")]
    pub results: Vec<crate::models::PackageWiki>,
    #[serde(rename = "cursor")]
    pub cursor: String,
    #[serde(rename = "has_more")]
    pub has_more: bool,
}

impl PackageWikiListResponse {
    pub fn new(results: Vec<crate::models::PackageWiki>, cursor: String, has_more: bool) -> PackageWikiListResponse {
        PackageWikiListResponse {
            results,
            cursor,
            has_more,
        }
    }
}


