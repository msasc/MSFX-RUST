use strum_macros::{EnumIter, Display};

#[derive(Debug, Clone, PartialEq, EnumIter, Display)]
pub enum Types {
    BOOLEAN,
    DECIMAL,
    FLOAT,
    INTEGER,
    LONG,
    DATE,
    TIME,
    TIMESTAMP,
    VARCHAR,
    CLOB,
    VARBINARY,
    BLOB,
    JSON,
}

impl Types {
}
