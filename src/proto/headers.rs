// BACKEND MESSAGES
pub const AUTHENTICATION: u8 = b'R';
pub const BACKEND_KEY_DATA: u8 = b'K';
pub const BIND_COMPLETE: u8 = b'2';
pub const CLOSE_COMPLETE: u8 = b'3';
pub const COMMAND_COMPLETE: u8 = b'C';
pub const COPY_IN_RESPONSE: u8 = b'G';
pub const COPY_OUT_RESPONSE: u8 = b'H';
pub const COPY_BOTH_RESPONSE: u8 = b'W';
pub const DATA_ROW: u8 = b'D';
pub const EMPTY_QUERY_RESPONSE: u8 = b'I';
pub const ERROR_RESPONSE: u8 = b'E';
pub const FUNCTION_CALL_RESPONSE: u8 = b'B';
pub const NEGOTIATE_PROTOCOL_VERSION: u8 = b'v';
pub const NO_DATA: u8 = b'n';
pub const NOTICE_RESPONSE: u8 = b'N';
pub const NOTIFICATION: u8 = b'A';
pub const PARAMETER_DESCRIPTION: u8 = b't';
pub const PARAMETER_STATUS: u8 = b'S';
pub const PARSE_COMPLETE: u8 = b'1';
pub const PORTAL_SUSPENDED: u8 = b's';
pub const READY_FOR_QUERY: u8 = b'Z';
pub const ROW_DESCRIPTION: u8 = b'T';

// FRONTEND MESSAGES
pub const BIND: u8 = b'B';
pub const CLOSE: u8 = b'C';
pub const COPY_FAIL: u8 = b'f';
pub const DESCRIBE: u8 = b'D';
pub const EXECUTE: u8 = b'E';
pub const FLUSH: u8 = b'H';
pub const FUNCTION_CALL: u8 = b'F';
pub const PARSE: u8 = b'P';
pub const PASSWORD_MESSAGE: u8 = b'p';
pub const QUERY: u8 = b'Q';
pub const SYNC: u8 = b'S';
pub const TERMINATE: u8 = b'X';
