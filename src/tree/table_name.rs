use super::name::Name;

#[derive(Debug, Clone)]
pub struct TableName {
    table_name: Name,
    prefix: TableNamePrefix,
}

impl TableName {
    pub fn new(name: String) -> TableName {
        return TableName {
            table_name: name,
            prefix: TableNamePrefix {
                catalog_name: "".to_string(),
                schema_name: "public".to_string(),
                explicit_catalog: false,
                explicit_schema: false,
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct TableNamePrefix {
    catalog_name: Name,
    schema_name: Name,
    explicit_catalog: bool,
    explicit_schema: bool,
}
