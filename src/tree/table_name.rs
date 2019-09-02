use super::name::Name;

pub struct TableName {
    table_name: Name,
    prefix: TableNamePrefix,
}

pub struct TableNamePrefix {
    catalog_name: Name,
    schema_name: Name,
    explicit_catalog: bool,
    explicit_schema: bool,
}
