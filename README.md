# Pebble Query

## What is Pebble Query

A simple helper library for SeaOrm to parse (applying filters, pagination, and sorting), execute, and return standard
DTOs with metadata.

## What it does

- Apply standard DTOs on SeaOrm `Select<T>`.
- Automatically filter by column fields and operators,
- sort by column fields, and
- paginate the query.
- Return standard DTOs with pagination information.

## Typical use case:

- Handle complex standard query request with multiple filters, sorting, and pagination from API clients, e.g. JSON
  request from web frontend.

In other words, Pebble Query provides some composable, generic functions that allow feeding SeaOrm the same standard
struct and expecting results wrapped in standard struct ready to be consumed. It can greatly simplify the common queries
and can be used for any SeaOrm entities without much boilerplate code.

## Example

For a sample database task:

- list all books by the three authors and filter by the book title contains "sea" and published after 1976. Sort by book
  title and return 10 books per page starting from page 2. Return all the data with pagination information.

You can do:

### 1. Without Pebble Query (pseudo-code)

A typical SeaOrm DSL query looks like this which is not easily reusable:

 ```rust
 let db = conn();
let book_ids: Vec<i32> = vec![1, 2, 3];

// construct query
let select: Select<book::Entity> = book::Entity::find()
                            .inner_join(author::Entity)
                            .filter(
                                Expr::col((document::Entity, document::Column::Id)).is_in(book_ids).into_condition() // filter by book ids
                            )
                            .filter(
                                Expr::col((book::Entity, book::Column::Title)) // contains "sea"
                            .contains("sea")
                                .and(Expr::col((book::Entity, book::Column::PublicationYear)) // published after 1976
                                    .gt(1976))
                            )
                            .order_by_asc(book::Column::Title)
                            .offset(10)
                            .limit(10);

let result: Vec<book::Model> = select.clone().all(db).await?;

// another trip to the database to get the pagination information such as total number of items and pages.
let pagination_info = select
                    .paginate(db, 10)
                    .num_items_and_pages().await?;

 ```

### 2. With Pebble Query

For each entity, in order to use Pebble Query, all that you need to write anew is a mapper between the queryable field
names and SeaOrm Columns. For example:

 ```rust
let book_column_map: HashMap<String, (book::Entity, book::Entity::Column) > = std::collections::HashMap::from([
                        ("id".to_string(), (book::Entity, book::Column::Id)),
                        ("title".to_string(), (book::Entity, book::Column::Title)),
                        ("publication_year".to_string(), (book::Entity, book::Column::PublicationYear)),
                        // and so on to map all the fields you want to be able to query
                    ].into_iter().map( | (k, v) | (k.to_string(), v)).collect();
 ```

 ```rust
 // then you can populate the standard SearchQuery DTO struct with the same query as above. Our example below is handwritten, but usually is generated and fed to SeaOrm backend. In fact, you can simply provide None for the `SearchQuery` parameter to Pebble Query and it will return all the results filtered by your initial `Select<T>`.
let query = SearchQuery {
        sort: Some(SearchSortOption {
        field: "title".to_string(),
        order: SortDirection::Asc as i32,
        }),
        offset: 10,
        length: 10,
        page: 0,
        filter: Some(SearchFilter {
        must: vec![
            SearchCondition {
                field: "title".to_string(),
                operator: SearchOperator::Contains as i32,
                value: Some("sea".to_string()),
                ..Default::default()
            },
            SearchCondition {
                field: "publication_year".to_string(),
                operator: SearchOperator::GreaterThan as i32,
                value: Some("1976".to_string()),
                ..Default::default()
            },
        ],
        ..Default::default ()
        }),
    ..Default::default ()
};
 ```

Then you can simply do:

 ```rust
 let db = conn();
let book_ids: Vec<i32> = vec![1, 2, 3];
let select: Select<Book> = book::Entity::find()
.inner_join(author::Entity)
.filter(
Expr::col((document::Entity, document::Column::Id)).is_in(book_ids).into_condition() // filter by book ids
);
let results: PebbleQueryResult<book::Model> = use_pebble_query(select, query, & BOOK_COLUMN_MAP, db).await?;
 ```

Or use fluent syntax:

 ```rust
let db = conn();
let result: PebbleQueryResult<book::Model> = book::Entity::find()
                                        .inner_join(author::Entity)
                                        .filter(
                                            Expr::col((document::Entity, document::Column::Id)).is_in(doc_ids).into_condition()
                                        )
                                        .pebble_query(query, & BOOK_COLUMN_MAP, db).await?; // add this to your existing `Select`.
 ```

The query result, with pagination information will be returned in these structs, ready to be returned to the frontend.

 ```rust
 // The result will contain the following information:
pub struct PebbleQueryResult<T: EntityTrait> {
    pub metadata: SearchResultMetadata,
    pub results: Vec<T::Model>,
}

pub struct SearchResultMetadata {
    pub result_items: i32,
    pub offset: i32,
    pub length: i32,
    pub page: i32,
    pub result_total_pages: i32,
    pub result_total_items: i32,
    pub query: Option<SearchQuery>,
    pub filter_count: Option<i32>,
    pub filter_reason: Option<String>,
}
 ```

And all of these are reusable. For most types of business-logic involving `book` entity, you can simply
attach `.pebble_query(query, &BOOK_COLUMN_MAP, db).await?` to your existing SeaOrm `Select` and expect the same
structured result.

For new entities, you only need to write a new mapper.

Some other middle util methods, e.g. convert SearchCondition to SeaOrm Condition, are also exposed which you can use to
reduce boilerplate code.

## Note:

1. This is not the most polished library. __Please do not use in production without reviewing the code and make
   necessary changes__.
2. You can get rid of the sample Struct and use your own DTOs. The `prost` dependency is only used for the sample DTOs,
   both of which can be removed.