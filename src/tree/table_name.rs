use super::name::Name;
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableName {
    pub table_name: Name,
    pub prefix: TableNamePrefix,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableNamePrefix {
    pub catalog_name: Name,
    pub schema_name: Name,
    pub explicit_catalog: bool,
    pub explicit_schema: bool,
}
