#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchQuery {
    /// When there is a relation present, all the other conditions are applied AFTER the relation filter is applied.
    #[prost(message, optional, tag = "9")]
    pub relation: ::core::option::Option<SearchRelation>,
    #[prost(message, optional, tag = "1")]
    pub sort: ::core::option::Option<SearchSortOption>,
    #[prost(int32, tag = "3")]
    pub offset: i32,
    #[prost(int32, tag = "4")]
    pub length: i32,
    #[prost(int32, tag = "5")]
    pub page: i32,
    #[prost(message, optional, tag = "6")]
    pub filter: ::core::option::Option<SearchFilter>,
    /// return only the first result.
    #[prost(bool, tag = "7")]
    pub find_one: bool,
    /// return all results
    #[prost(bool, tag = "8")]
    pub find_all: bool,
}
/// Find all the relations that are related to the object.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchRelation {
    #[prost(int32, repeated, tag = "1")]
    pub parent_ids: ::prost::alloc::vec::Vec<i32>,
    /// a string to indicate the type of the child object to filter
    #[prost(string, repeated, tag = "2")]
    pub child_type: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchFilter {
    /// a set of search conditions that must be satisfied all at once.
    #[prost(message, repeated, tag = "1")]
    pub must: ::prost::alloc::vec::Vec<SearchCondition>,
    /// a set of filters where at least one of the filters must be satisfied.
    #[prost(message, repeated, tag = "2")]
    pub any: ::prost::alloc::vec::Vec<SearchCondition>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchCondition {
    #[prost(string, tag = "1")]
    pub field: ::prost::alloc::string::String,
    #[prost(enumeration = "SearchOperator", tag = "2")]
    pub operator: i32,
    /// the optional parameter for determining matches for semantic similarity search.
    #[prost(float, optional, tag = "5")]
    pub threshold: ::core::option::Option<f32>,
    /// the only or first value.
    #[prost(string, optional, tag = "3")]
    pub value: ::core::option::Option<::prost::alloc::string::String>,
    /// the second value, to be used with BETWEEN, NOT_BETWEEN
    #[prost(string, optional, tag = "6")]
    pub value_to: ::core::option::Option<::prost::alloc::string::String>,
    /// a list of string value, to be used with IN, NOT_IN
    #[prost(string, repeated, tag = "4")]
    pub value_list: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Metadata about the query and the result returned for the query.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchResultMetadata {
    /// the actual count of the result contained in the response
    #[prost(int32, tag = "1")]
    pub result_items: i32,
    #[prost(int32, tag = "2")]
    pub offset: i32,
    #[prost(int32, tag = "3")]
    pub length: i32,
    #[prost(int32, tag = "4")]
    pub page: i32,
    #[prost(int32, tag = "5")]
    pub result_total_pages: i32,
    #[prost(int32, tag = "6")]
    pub result_total_items: i32,
    #[prost(message, optional, tag = "7")]
    pub query: ::core::option::Option<SearchQuery>,
    /// the count of results that are filtered out after the query in the post-processing process.
    /// For example, I queried fireflies, but there are 1000 textual objects but only 600 fireflies, then the filtered out count is 400, and the result count is 600, and the total result count is 1000
    /// Remember, this does not include the count of the results that are filtered out before the query, such as the ones that are not visible to the user.
    #[prost(int32, optional, tag = "9")]
    pub filter_count: ::core::option::Option<i32>,
    /// describe the reason
    #[prost(string, optional, tag = "10")]
    pub filter_reason: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchSortOption {
    #[prost(string, tag = "1")]
    pub field: ::prost::alloc::string::String,
    #[prost(enumeration = "SortDirection", tag = "2")]
    pub order: i32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SearchOperator {
    Unspecified = 0,
    /// SQL-like syntax
    Like = 21,
    NotLike = 22,
    /// SQL-ilike syntax
    Ilike = 23,
    NotIlike = 24,
    /// Semantic similarity, using cosine similarity.
    Similar = 27,
    Equals = 1,
    NotEquals = 2,
    GreaterThan = 3,
    GreaterThanOrEquals = 4,
    LessThan = 5,
    LessThanOrEquals = 6,
    In = 7,
    NotIn = 8,
    Contains = 9,
    NotContains = 10,
    StartsWith = 11,
    NotStartsWith = 12,
    EndsWith = 13,
    NotEndsWith = 14,
    Exists = 15,
    NotExists = 16,
    IsNull = 17,
    IsNotNull = 18,
    IsTrue = 19,
    IsFalse = 20,
    Between = 25,
    NotBetween = 26,
}
impl SearchOperator {
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SearchOperator::Unspecified => "SEARCH_OPERATOR_UNSPECIFIED",
            SearchOperator::Like => "SEARCH_OPERATOR_LIKE",
            SearchOperator::NotLike => "SEARCH_OPERATOR_NOT_LIKE",
            SearchOperator::Ilike => "SEARCH_OPERATOR_ILIKE",
            SearchOperator::NotIlike => "SEARCH_OPERATOR_NOT_ILIKE",
            SearchOperator::Similar => "SEARCH_OPERATOR_SIMILAR",
            SearchOperator::Equals => "SEARCH_OPERATOR_EQUALS",
            SearchOperator::NotEquals => "SEARCH_OPERATOR_NOT_EQUALS",
            SearchOperator::GreaterThan => "SEARCH_OPERATOR_GREATER_THAN",
            SearchOperator::GreaterThanOrEquals => {
                "SEARCH_OPERATOR_GREATER_THAN_OR_EQUALS"
            }
            SearchOperator::LessThan => "SEARCH_OPERATOR_LESS_THAN",
            SearchOperator::LessThanOrEquals => "SEARCH_OPERATOR_LESS_THAN_OR_EQUALS",
            SearchOperator::In => "SEARCH_OPERATOR_IN",
            SearchOperator::NotIn => "SEARCH_OPERATOR_NOT_IN",
            SearchOperator::Contains => "SEARCH_OPERATOR_CONTAINS",
            SearchOperator::NotContains => "SEARCH_OPERATOR_NOT_CONTAINS",
            SearchOperator::StartsWith => "SEARCH_OPERATOR_STARTS_WITH",
            SearchOperator::NotStartsWith => "SEARCH_OPERATOR_NOT_STARTS_WITH",
            SearchOperator::EndsWith => "SEARCH_OPERATOR_ENDS_WITH",
            SearchOperator::NotEndsWith => "SEARCH_OPERATOR_NOT_ENDS_WITH",
            SearchOperator::Exists => "SEARCH_OPERATOR_EXISTS",
            SearchOperator::NotExists => "SEARCH_OPERATOR_NOT_EXISTS",
            SearchOperator::IsNull => "SEARCH_OPERATOR_IS_NULL",
            SearchOperator::IsNotNull => "SEARCH_OPERATOR_IS_NOT_NULL",
            SearchOperator::IsTrue => "SEARCH_OPERATOR_IS_TRUE",
            SearchOperator::IsFalse => "SEARCH_OPERATOR_IS_FALSE",
            SearchOperator::Between => "SEARCH_OPERATOR_BETWEEN",
            SearchOperator::NotBetween => "SEARCH_OPERATOR_NOT_BETWEEN",
        }
    }
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SEARCH_OPERATOR_UNSPECIFIED" => Some(Self::Unspecified),
            "SEARCH_OPERATOR_LIKE" => Some(Self::Like),
            "SEARCH_OPERATOR_NOT_LIKE" => Some(Self::NotLike),
            "SEARCH_OPERATOR_ILIKE" => Some(Self::Ilike),
            "SEARCH_OPERATOR_NOT_ILIKE" => Some(Self::NotIlike),
            "SEARCH_OPERATOR_SIMILAR" => Some(Self::Similar),
            "SEARCH_OPERATOR_EQUALS" => Some(Self::Equals),
            "SEARCH_OPERATOR_NOT_EQUALS" => Some(Self::NotEquals),
            "SEARCH_OPERATOR_GREATER_THAN" => Some(Self::GreaterThan),
            "SEARCH_OPERATOR_GREATER_THAN_OR_EQUALS" => Some(Self::GreaterThanOrEquals),
            "SEARCH_OPERATOR_LESS_THAN" => Some(Self::LessThan),
            "SEARCH_OPERATOR_LESS_THAN_OR_EQUALS" => Some(Self::LessThanOrEquals),
            "SEARCH_OPERATOR_IN" => Some(Self::In),
            "SEARCH_OPERATOR_NOT_IN" => Some(Self::NotIn),
            "SEARCH_OPERATOR_CONTAINS" => Some(Self::Contains),
            "SEARCH_OPERATOR_NOT_CONTAINS" => Some(Self::NotContains),
            "SEARCH_OPERATOR_STARTS_WITH" => Some(Self::StartsWith),
            "SEARCH_OPERATOR_NOT_STARTS_WITH" => Some(Self::NotStartsWith),
            "SEARCH_OPERATOR_ENDS_WITH" => Some(Self::EndsWith),
            "SEARCH_OPERATOR_NOT_ENDS_WITH" => Some(Self::NotEndsWith),
            "SEARCH_OPERATOR_EXISTS" => Some(Self::Exists),
            "SEARCH_OPERATOR_NOT_EXISTS" => Some(Self::NotExists),
            "SEARCH_OPERATOR_IS_NULL" => Some(Self::IsNull),
            "SEARCH_OPERATOR_IS_NOT_NULL" => Some(Self::IsNotNull),
            "SEARCH_OPERATOR_IS_TRUE" => Some(Self::IsTrue),
            "SEARCH_OPERATOR_IS_FALSE" => Some(Self::IsFalse),
            "SEARCH_OPERATOR_BETWEEN" => Some(Self::Between),
            "SEARCH_OPERATOR_NOT_BETWEEN" => Some(Self::NotBetween),
            _ => None,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SortDirection {
    Unspecified = 0,
    Asc = 1,
    Desc = 2,
}
impl SortDirection {
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SortDirection::Unspecified => "SORT_DIRECTION_UNSPECIFIED",
            SortDirection::Asc => "SORT_DIRECTION_ASC",
            SortDirection::Desc => "SORT_DIRECTION_DESC",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SORT_DIRECTION_UNSPECIFIED" => Some(Self::Unspecified),
            "SORT_DIRECTION_ASC" => Some(Self::Asc),
            "SORT_DIRECTION_DESC" => Some(Self::Desc),
            _ => None,
        }
    }
}
