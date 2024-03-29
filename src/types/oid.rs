use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Oid {
    Bool = 16,
    Bytea = 17,
    Char = 18,
    Name = 19,
    Int8 = 20,
    Int2 = 21,
    Int2Vector = 22,
    Int4 = 23,
    RegProc = 24,
    Text = 25,
    Oid = 26,
    Tid = 27,
    Xid = 28,
    Cid = 29,
    OidVector = 30,
    PgDdlCommand = 32,
    PgType = 71,
    PgAttribute = 75,
    PgProc = 81,
    PgClass = 83,
    Json = 114,
    Xml = 142,
    XmlArray = 143,
    PgNodeTree = 194,
    JsonArray = 199,
    Smgr = 210,
    IndexAmHandler = 325,
    Point = 600,
    Lseg = 601,

    Unknown = 705,

    Varbit = 1562,
}