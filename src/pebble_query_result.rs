use crate::pebble_converter::{PebbleConverter, PebbleConverterTrait};
use crate::sample_dto_structs::{SearchQuery, SearchResultMetadata};

use sea_orm::{EntityTrait, ItemsAndPagesNumber};
use serde::{Deserialize, Serialize};

// implement the generically typed PebbleQueryResult
impl<T: EntityTrait> PebbleQueryResult<T> {
    pub fn from(
        query: SearchQuery,
        results: Vec<T::Model>,
        items_and_pages_num: ItemsAndPagesNumber,
    ) -> Self {
        let metadata = PebbleConverter::query_many_result_to_standard_query_result::<T>(
            results.len() as i32,
            items_and_pages_num.number_of_items as i32,
            items_and_pages_num.number_of_pages as i32,
            query,
        );
        PebbleQueryResult { metadata, results }
    }
}

/// Standard Pebble Query Result
pub struct PebbleQueryResult<T: EntityTrait> {
    pub metadata: SearchResultMetadata,
    pub results: Vec<T::Model>,
}

//
pub trait PebbleQueryResultUtilTrait<T: EntityTrait> {
    fn first(&self) -> Option<&T::Model>;
    /// This moves and maps a QueryResult for SeaOrm entity to a generic QueryResult for any type.
    /// This is useful for picking out result fields from the query result.
    ///
    /// # Arguments
    ///
    /// * `result_filter_map`:
    /// * `filter_out_reason`: Describe what are filtered out.
    ///
    /// returns: PebbleQueryResultGeneric<U>
    fn map_into_generic<U>(
        self,
        result_filter_map: fn(T::Model) -> Option<U>,
        filter_out_reason: Option<String>,
    ) -> PebbleQueryResultGeneric<U>;
}

impl<T: EntityTrait> PebbleQueryResultUtilTrait<T> for PebbleQueryResult<T> {
    fn first(&self) -> Option<&T::Model> {
        self.results.first()
    }

    fn map_into_generic<U>(
        self,
        result_filter_map: fn(T::Model) -> Option<U>,
        filter_reason: Option<String>,
    ) -> PebbleQueryResultGeneric<U> {
        let mut results: Vec<U> = Vec::new();

        let initial_result_count = self.results.len() as u64;
        for filtered_result in self.results {
            if let Some(mapped_result) = result_filter_map(filtered_result) {
                results.push(mapped_result);
            }
        }
        let filtered_result_count = results.len() as u64;
        let filtered_out_count = initial_result_count.saturating_sub(filtered_result_count);
        // update the results count in the metadata
        let metadata = SearchResultMetadata {
            result_items: results.len() as i32,
            filter_count: Some(filtered_out_count as i32),
            filter_reason,
            ..self.metadata
        };
        PebbleQueryResultGeneric { metadata, results }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PebbleQueryResultGeneric<T> {
    pub metadata: SearchResultMetadata,
    pub results: Vec<T>,
}
pub trait PebbleQueryResultGenericUtilTraits<O> {
    fn first(&self) -> Option<&O>;

    fn map_filter_result<TARGET>(
        self,
        result_filter_map: fn(O) -> Option<TARGET>,
        filter_out_reason: Option<String>,
    ) -> PebbleQueryResultGeneric<TARGET>;
}

impl<O> PebbleQueryResultGenericUtilTraits<O> for PebbleQueryResultGeneric<O> {
    fn first(&self) -> Option<&O> {
        self.results.first()
    }

    fn map_filter_result<TARGET>(
        self,
        result_filter_map: fn(O) -> Option<TARGET>,
        filter_out_reason: Option<String>,
    ) -> PebbleQueryResultGeneric<TARGET> {
        let mut results: Vec<TARGET> = Vec::new();

        let initial_result_count = self.results.len() as u64;
        for filtered_result in self.results {
            if let Some(mapped_result) = result_filter_map(filtered_result) {
                results.push(mapped_result);
            }
        }
        let filtered_result_count = results.len() as u64;
        let filtered_out_count = initial_result_count.saturating_sub(filtered_result_count);
        // update the results count in the metadata
        let metadata = SearchResultMetadata {
            result_items: results.len() as i32,
            filter_count: Some(filtered_out_count as i32),
            filter_reason: filter_out_reason,
            ..self.metadata
        };
        PebbleQueryResultGeneric { metadata, results }
    }
}
