use std::collections::HashMap;

use crate::sample_dto_structs::SearchQuery;
use sea_orm::prelude::async_trait::async_trait;
use sea_orm::EntityTrait;
use sea_orm::FromQueryResult;
use sea_orm::QueryFilter;
use sea_orm::Select;
use sea_orm::{ConnectionTrait, DbErr};
use sea_orm::{QuerySelect, QueryTrait};

use crate::pebble_query_result::PebbleQueryResult;
use crate::pebble_utils::{
    add_sort_to_select, get_query_pagination_numbers, query_to_seaorm_conditions,
};

/// Apply SearchQuery to a SearOrm Select<Entity> with conditions
///
/// This is a util to convert a SearchQuery to a SeaOrm Select<Entity> with conditions.
///
/// To use simply, simply call it after you called SeaOrm select methods such as `find()` or `find_related()`.
///
/// This method will take care of the rest, such as applying conditions, pagination, sorting etc and return a Select with all these conditions applied and ready to be executed.
///
///
/// # Arguments
///
/// * `query`: StandardQuery object.
/// * `field_to_column_map`: User-provided map of string field name to SeaOrm Column. This is used to convert the field name in the `sq_filter` to SeaOrm Column.
/// * `select_with_conditions`: The select query with conditions applied. This select is often cloned before it was first consumed to get the data back.
///
/// In order to avoid ambiguous id error, put the tuple `(annotation::Entity, annotation::Column::Id)` (which is converted into ``annotation`.`id``), not `annotation::Column::Id` in the map.
///
/// # Returns
///
/// * `Condition`: The condition for `must` filters, i.e. `Condition::all().add`
/// * `Condition`: The condition for `any` filters, i.e. `Condition::any().add`
///
/// See https://www.sea-ql.org/SeaORM/docs/advanced-query/conditional-expression/
pub fn query_to_select<E, M>(
    query: &SearchQuery,
    field_to_column_map: &HashMap<String, (E, <E as EntityTrait>::Column)>,
    select_with_conditions: Select<E>,
) -> Select<E>
where
    E: EntityTrait<Model = M>,
{
    let (must_conditions, any_conditions) = query_to_seaorm_conditions(query, field_to_column_map);

    let mut select_with_conditions = select_with_conditions
        .apply_if(
            if !must_conditions.is_empty() {
                Some(must_conditions.clone())
            } else {
                None
            },
            |query, condition| query.filter(condition),
        )
        .apply_if(
            if !any_conditions.is_empty() {
                Some(any_conditions.clone())
            } else {
                None
            },
            |query, condition| query.filter(condition),
        );
    if query.length > 0 {
        select_with_conditions = select_with_conditions.limit(query.length as u64)
    }

    if query.offset > 0 {
        select_with_conditions = select_with_conditions.offset(query.offset as u64)
    }

    let query_sort = query.sort.clone();
    if query_sort.is_some() {
        select_with_conditions = add_sort_to_select(
            select_with_conditions,
            query_sort.unwrap(),
            field_to_column_map,
        );
    };
    select_with_conditions
}

/// # Apply and run SearchQuery to a SearOrm Select<Entity> with conditions
///
/// This is the main method unless more conditions are needed. The method does three things.
///
/// 1. It first takes in an initial Select<Entity>, e.g. `Entity::find()`, and apply the filtering conditions, pagination, and sorting to the initial select by calling `query_to_select`.
/// 2. Then, it execute the select with conditions applied twice, first to get the results, second to get pagination information.
/// 3. Finally, this wrapped the data and metadata into a PebbleQueryResult and return it.
///
/// # Arguments
/// * `initial_select`: The initial select query with conditions applied. This select is often cloned before it was first consumed to get the data back.
/// * `field_to_column_map`: User-provided map of string field name to SeaOrm Column in the form of `Hashmap<String, (Entity, Entity::Column)>`. This is used to convert the field name in the `sq_filter` to SeaOrm Column.
/// * `query`: Option<SearchQuery> object - the query to be applied to the select. If none, all entity entries will be returned.
/// * `db`: The database connection.
///
/// # Returns
/// * `PebbleQueryResult`: The result of the query.
///
/// # Example
///
/// ```rust
///
/// // Find all annotations for a document
/// pub async fn query_annotations_by_documents(&self, doc_ids: Vec<i32>, query: Option<SearchQuery>) -> Result<
///         PebbleQueryResultGeneric<CommonAnnotation>, CarrelOrmErr> {
///         let db = self.conn();
///
///         let select = annotation::Entity::find()
///   .inner_join(document::Entity)
///   .filter(
///       Expr::col( (document::Entity, document::Column::Id)).is_in(doc_ids).into_condition()
///   ); // any select can be used as the initial condition
///
///         let result = run_query(db, query, &ANNOTATION_COLUMN_MAP, select).await?;
///
///         Ok(result.map_into_generic(|annotation| Some(annotation.into_common_type()), None))
///     }
/// ```
pub async fn use_pebble_query<C, E, M>(
    initial_select: Select<E>,
    query: Option<SearchQuery>,
    field_to_column_map: &HashMap<String, (E, <E as EntityTrait>::Column)>,
    db: &C,
) -> Result<PebbleQueryResult<E>, DbErr>
where
    C: ConnectionTrait,
    E: EntityTrait<Model = M>,
    M: FromQueryResult + Sized + Send + Sync,
{
    let query = query.unwrap_or(SearchQuery {
        find_all: true,
        ..Default::default()
    });
    // parse query and add all contions, pagination, and sortings to the initial selection and return the modified selected.
    let select_with_conditions = query_to_select(&query, field_to_column_map, initial_select);

    let query_result: Vec<<E as EntityTrait>::Model>;

    // check find_one and find_all fields in the SearchQuery before execute the queries receive data back.
    if query.find_one {
        let result = select_with_conditions.clone().one(db).await?;
        // if exists return the result in Vec, else return empty Vec
        if result.is_some() {
            let result = result.unwrap();
            query_result = vec![result]
        } else {
            query_result = vec![]
        }
    } else {
        let result = select_with_conditions.clone().all(db).await?;
        query_result = result
    }

    // use the same (cloned) query to execute for a second time purely to get the total counts and page numbers using the same conditions.
    let total_items_and_pages_number =
        get_query_pagination_numbers(db, &query, select_with_conditions).await?;

    // construct the pebble_query_result from the data (first run) and the metadata (second run) from the DB.
    let pebble_query_result =
        PebbleQueryResult::from(query, query_result, total_items_and_pages_number);

    Ok(pebble_query_result)
}

#[async_trait]
pub trait RunQueryExt<C, E, M>
where
    C: ConnectionTrait,
    E: EntityTrait<Model = M>,
    M: Sized + Send + Sync + FromQueryResult,
{
    async fn pebble_query(
        self,
        query: Option<SearchQuery>,
        field_to_column_map: &HashMap<String, (E, <E as EntityTrait>::Column)>,
        db: &C,
    ) -> Result<PebbleQueryResult<E>, DbErr>;
}

#[async_trait]
impl<C, E, M> RunQueryExt<C, E, M> for Select<E>
where
    C: ConnectionTrait,
    E: EntityTrait<Model = M>,
    M: Sized + Send + Sync + FromQueryResult,
{
    async fn pebble_query(
        self,
        query: Option<SearchQuery>,
        field_to_column_map: &HashMap<String, (E, <E as EntityTrait>::Column)>,
        db: &C,
    ) -> Result<PebbleQueryResult<E>, DbErr> {
        use_pebble_query(self, query, field_to_column_map, db).await
    }
}
