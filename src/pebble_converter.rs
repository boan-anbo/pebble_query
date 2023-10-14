use crate::sample_dto_structs::{SearchQuery, SearchResultMetadata};

use sea_orm::EntityTrait;

pub struct PebbleConverter {}

pub trait PebbleConverterTrait {
    fn query_many_result_to_standard_query_result<T: EntityTrait>(
        result_items: i32,
        result_total_items: i32,
        result_total_pages: i32,
        query: SearchQuery,
    ) -> SearchResultMetadata;
}

impl PebbleConverterTrait for PebbleConverter {
    fn query_many_result_to_standard_query_result<T: EntityTrait>(
        result_items: i32,
        result_total_items: i32,
        result_total_pages: i32,
        query: SearchQuery,
    ) -> SearchResultMetadata {
        SearchResultMetadata {
            result_items,
            offset: query.offset,
            length: query.length,
            page: query.page,
            result_total_pages,
            result_total_items,
            query: Some(query),

            filter_count: None,
            filter_reason: None,
        }
    }
}
