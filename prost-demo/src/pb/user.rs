#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Eq)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct User {
    #[prost(int32, tag = "1")]
    #[serde(rename = "in")]
    pub age: i32,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
}
