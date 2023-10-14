use std::collections::HashMap;

use crate::sample_dto_structs::{
    SearchCondition, SearchOperator, SearchQuery, SearchSortOption, SortDirection,
};
use convert_case::{Case, Casing};
use sea_orm::prelude::Expr;
use sea_orm::{
    ColumnTrait, Condition, ConnectionTrait, DbErr, EntityTrait, FromQueryResult,
    ItemsAndPagesNumber, Linked, ModelTrait, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect,
    QueryTrait, Select,
};

use crate::errors::PebbleQueryError;
use crate::errors::PebbleQueryError::{InvalidOperator, MissingValue};

/// Add sort option to the Select and return the new Select.
pub fn add_sort_to_select<T: ColumnTrait, Entity: EntityTrait>(
    select_entity: Select<Entity>,
    sort_condition: SearchSortOption,
    field_to_column_map: &HashMap<String, (Entity, T)>,
) -> Select<Entity> {
    let mut select_with_sort = select_entity;

    let column_tuple: (Entity, T) = *field_to_column_map
        .iter()
        .find(|(field_name, _column)| (field_name == &sort_condition.field.as_str()))
        .unwrap_or_else(|| {
            panic!(
                "Column {} not provided in field_to_column_map",
                sort_condition.field.as_str()
            )
        })
        .1;

    let column = column_tuple.1;

    match SortDirection::try_from(sort_condition.order).unwrap() {
        SortDirection::Unspecified => {
            select_with_sort = select_with_sort.order_by_asc(column);
        }
        SortDirection::Asc => {
            select_with_sort = select_with_sort.order_by_asc(column);
        }
        SortDirection::Desc => {
            select_with_sort = select_with_sort.order_by_desc(column);
        }
    }

    select_with_sort
}

// Add final condition
pub fn add_query_condition_to_sea_orm_condition<E>(
    input_current_condition: Condition,
    filter: &SearchCondition,
    column: (E, E::Column),
) -> Result<Condition, PebbleQueryError>
where
    E: EntityTrait,
{
    let value = filter.value.clone().unwrap_or_else(|| "".to_string());
    let value_list = filter.value_list.clone();
    let value_to = filter.value_to.clone();
    let result = add_condition::<E>(
        input_current_condition,
        filter.operator,
        &column,
        value.as_str(),
        value_list,
        value_to,
    )?;

    Ok(result)
}

/// Construct a new condition based on `operator`, `column`, `value`, `value_list`, `value_to` and add it to the `input_condition`.
/// Returns the new condition.
/// This is very incomplete and does not take SQL backend differences into account. Adjust as needed.
fn add_condition<E>(
    input_condition: Condition,
    operator: i32,
    column_tuple: &(E, E::Column),
    value: &str,
    value_list: Vec<String>,
    value_to: Option<String>,
) -> Result<Condition, PebbleQueryError>
where
    E: EntityTrait,
{
    let condition = match SearchOperator::try_from(operator).unwrap_or(SearchOperator::Unspecified)
    {
        SearchOperator::Contains => {
            let value: &str = value;

            input_condition.add(Expr::col(*column_tuple).like(format!("%{}%", value)))
        }
        SearchOperator::Equals => input_condition.add(Expr::col(*column_tuple).eq(value)),
        SearchOperator::GreaterThan => input_condition.add(Expr::col(*column_tuple).gt(value)),
        SearchOperator::GreaterThanOrEquals => {
            input_condition.add(Expr::col(*column_tuple).gte(value))
        }
        SearchOperator::Like => input_condition.add(Expr::col(*column_tuple).like(value)),
        SearchOperator::LessThan => input_condition.add(Expr::col(*column_tuple).lt(value)),
        SearchOperator::LessThanOrEquals => {
            input_condition.add(Expr::col(*column_tuple).lte(value))
        }
        SearchOperator::NotEquals => input_condition.add(Expr::col(*column_tuple).ne(value)),
        SearchOperator::In => input_condition.add(Expr::col(*column_tuple).is_in(value_list)),
        SearchOperator::NotIn => {
            input_condition.add(Expr::col(*column_tuple).is_not_in(value_list))
        }
        SearchOperator::IsNull => input_condition.add(Expr::col(*column_tuple).is_null()),
        SearchOperator::IsNotNull => input_condition.add(Expr::col(*column_tuple).is_not_null()),
        // SearchOperator::StartsWith => {
        //     input_condition.add(Expr::col(*column_tuple).starts_with(value)) // These are commented because they are currently not supported by SeaORM 12's `Expr`.
        // }
        // SearchOperator::EndsWith => input_condition.add(Expr::col(*column_tuple).ends_with(value)),
        SearchOperator::Between => {
            if !value.is_empty() {
                if let Some(value_to) = value_to {
                    input_condition.add(Expr::col(*column_tuple).between(value, value_to.as_str()))
                } else {
                    return Err(MissingValue("value_to is required for between".to_string()));
                }
            } else {
                return Err(MissingValue("value is required for between".to_string()));
            }
        }
        SearchOperator::NotBetween => {
            if !value.is_empty() {
                if let Some(value_to) = value_to {
                    input_condition
                        .add(Expr::col(*column_tuple).not_between(value, value_to.as_str()))
                } else {
                    return Err(MissingValue(
                        "value_to is required for not between".to_string(),
                    ));
                }
            } else {
                return Err(MissingValue(
                    "value is required for not between".to_string(),
                ));
            }
        }
        _ => {
            return Err(InvalidOperator(format!("Invalid operator: {}", operator)));
        }
    };
    Ok(condition)
}

/// Traverse all query fields and normalize them to snake case.
///
/// assert_eq!("my_variable_name", "My variable NAME".to_case(Case::Snake)
pub fn normalize_query(input_query: &SearchQuery) -> SearchQuery {
    let mut query = input_query.clone();
    if query.filter.is_some() {
        query.filter = query.filter.clone().map(|mut filter| {
            filter.must = filter
                .must
                .into_iter()
                .map(|mut condition| {
                    condition.field = condition.field.to_case(Case::Snake);
                    condition
                })
                .collect();
            filter.any = filter
                .any
                .into_iter()
                .map(|mut condition| {
                    condition.field = condition.field.to_case(Case::Snake);
                    condition
                })
                .collect();
            filter
        });
    }

    if query.sort.is_some() {
        query.sort = query.sort.clone().map(|mut sort| {
            sort.field = sort.field.to_case(Case::Snake);
            sort
        });
    }
    query
}

pub static DEFAULT_PAGE_SIZE: u64 = 25;

/// Get the pagination information from the query.
///
/// Typically runs after a paginated query is executed and results returned. If pagination information is needed, use the same select to run this again to get it.
///
/// # Arguments
///
/// * `query`: The query to get the pagination information from which has conditions pagesize and page number etc.
/// * `select_with_conditions`: The select query with conditions applied. This select is often cloned before it was first consumed to get the data back.
///
/// # Returns
///
/// * `ItemsAndPagesNumber`: The pagination information.
pub async fn get_query_pagination_numbers<E, C, M>(
    db: &C,
    query: &SearchQuery,
    select_with_conditions: Select<E>,
) -> Result<ItemsAndPagesNumber, DbErr>
where
    E: EntityTrait<Model = M>,
    M: FromQueryResult + Sized + Send + Sync,
    C: ConnectionTrait,
{
    let total_items_and_pages_number = select_with_conditions
        .paginate(
            db,
            // page size, if the current value is 0, then set it to 25
            match query.length > 0 {
                true => query.length as u64,
                false => DEFAULT_PAGE_SIZE,
            },
        )
        .num_items_and_pages()
        .await?;
    Ok(total_items_and_pages_number)
}

pub fn apply_conditions_to_select<E, M>(
    query: &SearchQuery,
    field_to_column_map: &HashMap<String, (E, <E as EntityTrait>::Column)>,
) -> Select<E>
where
    E: EntityTrait<Model = M>,
    M: FromQueryResult + Sized + Send + Sync,
{
    let (current_must_condition, current_any_condition) =
        query_to_seaorm_conditions::<E>(query, field_to_column_map);

    let select_with_conditions: Select<E> = match query.find_all {
        true => E::find(),
        false => match query.find_one {
            true => E::find(), // if the query asks for all and one, then adds no conditions. Later the query itself will only pick one among all entities.
            false => {
                E::find() // if the query asks neither for find_all or find_one, then adds both conditions.
                    .apply_if(
                        if !current_must_condition.is_empty() {
                            Some(current_must_condition.clone())
                        } else {
                            None
                        },
                        |query, condition| query.filter(condition),
                    )
                    .apply_if(
                        if !current_any_condition.is_empty() {
                            Some(current_any_condition.clone())
                        } else {
                            None
                        },
                        |query, condition| query.filter(condition),
                    )
            }
        },
    };
    select_with_conditions
}
pub fn apply_linked_conditions_to_select<E, L, M>(
    entity: &M,
    query: &SearchQuery,
    field_to_column_map: &HashMap<String, (L::ToEntity, <L::ToEntity as EntityTrait>::Column)>,
    linked: L,
) -> Select<L::ToEntity>
where
    E: EntityTrait<Model = M>,
    M: ModelTrait<Entity = E> + Sized + Send + Sync + FromQueryResult, // Ensure M::Entity is E
    L: Linked<FromEntity = E>,
    L::ToEntity: EntityTrait,
{
    let (current_must_condition, current_any_condition) =
        query_to_seaorm_conditions::<L::ToEntity>(query, field_to_column_map);

    let select_with_conditions: Select<<L as Linked>::ToEntity> = match query.find_all {
        true => entity.find_linked(linked),
        false => match query.find_one {
            true => entity.find_linked(linked),
            // if the query asks for all and one, then adds no conditions. Later the query itself will only pick one among all entities.
            // if the query asks neither for find_all or find_one, then adds both conditions.
            false => entity
                .find_linked(linked)
                .apply_if(
                    if !current_must_condition.is_empty() {
                        Some(current_must_condition.clone())
                    } else {
                        None
                    },
                    |query, condition| query.filter(condition),
                )
                .apply_if(
                    if !current_any_condition.is_empty() {
                        Some(current_any_condition.clone())
                    } else {
                        None
                    },
                    |query, condition| query.filter(condition),
                ),
        },
    };
    select_with_conditions
}

pub fn query_to_seaorm_conditions<E: EntityTrait>(
    query: &SearchQuery,
    field_to_column_map: &HashMap<String, (E, <E>::Column)>,
) -> (Condition, Condition) {
    let query = normalize_query(query);
    let mut current_must_condition = Condition::all();
    let mut current_any_condition = Condition::any();

    let query_filter = query.filter.clone();
    if let Some(filter_set) = query_filter {
        for filter in filter_set.must {
            current_must_condition =
                extract_query_conditions::<E>(current_must_condition, filter, field_to_column_map)
                    .unwrap();
        }

        for filter in filter_set.any {
            current_any_condition =
                extract_query_conditions::<E>(current_any_condition, filter, field_to_column_map)
                    .unwrap();
        }
    };
    (current_must_condition, current_any_condition)
}

/// Extract conditions from SearchQuery's FilterSet and returns SeaOrm Condition ready to be used in Find selector.
///
/// # Arguments
///
/// * `base_sea_or_condition`: Extract all conditions from StandardQuery Condtions and convert them to SeaOrm Condition chain with the help of `field_column_map`.
///
/// The base condition is used to chain the conditions with `and` or `or` operator.
///
/// * `sq_filter`: a StandardQuery Filter, e.g.
///
/// ## Example:
///
/// ```
/// use carrel_commons::generic::api::query::v1::{Condition, Operator};
///
///     fn main() {
///
///      let sample_sq_filter_is_1 = carrel_commons::generic::api::query::v1::Condition {
///! field: "id".to_string(),
///! operator: Operator::Eq,
///! value: Some("1".to_string()),
///! value_list: vec![], // for operators like `in` or `not in`
///! value_to: None, // for operators like `between` or `not between`
///      }; // this translate to `id = 1`
///
///     let sample_sq_filter_is_in_1_2_3 = carrel_commons::generic::api::query::v1::Condition {
///! field: "id".to_string(),
///! operator: i32::from(Operator::In),
///! value: None,
///! value_list: vec!["1".to_string(), "2".to_string(), "3".to_string()],
///! value_to: None,
///      }; // this translate to `id in (1, 2, 3)`
///
///     let sample_sq_filter_is_between_1_to_10 = carrel_commons::generic::api::query::v1::Condition {
///! field: "id".to_string(),
///! operator: i32::from(Operator::Between),
///! value: Some("1".to_string()),
///! value_list: vec![],
///! value_to: Some("10".to_string()),
///
///     }; // this translate to `id between 1 and 10`
///
///     }
///
///
///
/// ```
///
/// * `field_column_map`: User-provided map of string field name to SeaOrm Column. This is used to convert the field name in the `sq_filter` to SeaOrm Column.
///
/// ## Example:
///
/// ```
/// use sea_orm::tests_cfg::cake;
///
///     let map = std::collections::HashMap::from([
///    ("id".to_string(), (cake::Entity, cake::Column::Id)),
///    ("name".to_string(), (cake::Entity, cake::Column::Name)),
///    ("price".to_string(), (cake::Entity,cake::Column::Price)),
///    // ... and so on.
///
///     ]);
///
/// ```
fn extract_query_conditions<E>(
    base_sea_orm_condition: Condition,
    sq_filter: SearchCondition,
    field_column_map: &HashMap<String, (E, E::Column)>,
) -> Result<Condition, DbErr>
where
    E: EntityTrait,
{
    // use the provided column name to Expr(Entity, Entity::Column) map to construct a condition
    let mapped_column: (E, E::Column) = *field_column_map
        .iter()
        .find(|(field_name, _column)| (field_name == &sq_filter.field.as_str()))
        .expect(
            format!(
                "Column \"{}\" not provided in field_to_column_map",
                sq_filter.field.as_str() // throw if
            )
            .as_str(),
        )
        .1;

    let result = add_query_condition_to_sea_orm_condition::<E>(
        base_sea_orm_condition,
        &sq_filter,
        mapped_column,
    )
    .unwrap();
    Ok(result)
}

