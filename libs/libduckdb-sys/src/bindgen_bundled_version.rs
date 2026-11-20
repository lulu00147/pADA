pub const DUCKDB_TYPE_DUCKDB_TYPE_INVALID: DUCKDB_TYPE = 0;
pub const DUCKDB_TYPE_DUCKDB_TYPE_BOOLEAN: DUCKDB_TYPE = 1;
pub const DUCKDB_TYPE_DUCKDB_TYPE_TINYINT: DUCKDB_TYPE = 2;
pub const DUCKDB_TYPE_DUCKDB_TYPE_SMALLINT: DUCKDB_TYPE = 3;
pub const DUCKDB_TYPE_DUCKDB_TYPE_INTEGER: DUCKDB_TYPE = 4;
pub const DUCKDB_TYPE_DUCKDB_TYPE_BIGINT: DUCKDB_TYPE = 5;
pub const DUCKDB_TYPE_DUCKDB_TYPE_UTINYINT: DUCKDB_TYPE = 6;
pub const DUCKDB_TYPE_DUCKDB_TYPE_USMALLINT: DUCKDB_TYPE = 7;
pub const DUCKDB_TYPE_DUCKDB_TYPE_UINTEGER: DUCKDB_TYPE = 8;
pub const DUCKDB_TYPE_DUCKDB_TYPE_UBIGINT: DUCKDB_TYPE = 9;
pub const DUCKDB_TYPE_DUCKDB_TYPE_FLOAT: DUCKDB_TYPE = 10;
pub const DUCKDB_TYPE_DUCKDB_TYPE_DOUBLE: DUCKDB_TYPE = 11;
pub const DUCKDB_TYPE_DUCKDB_TYPE_TIMESTAMP: DUCKDB_TYPE = 12;
pub const DUCKDB_TYPE_DUCKDB_TYPE_DATE: DUCKDB_TYPE = 13;
pub const DUCKDB_TYPE_DUCKDB_TYPE_TIME: DUCKDB_TYPE = 14;
pub const DUCKDB_TYPE_DUCKDB_TYPE_INTERVAL: DUCKDB_TYPE = 15;
pub const DUCKDB_TYPE_DUCKDB_TYPE_HUGEINT: DUCKDB_TYPE = 16;
pub const DUCKDB_TYPE_DUCKDB_TYPE_UHUGEINT: DUCKDB_TYPE = 32;
pub const DUCKDB_TYPE_DUCKDB_TYPE_VARCHAR: DUCKDB_TYPE = 17;
pub const DUCKDB_TYPE_DUCKDB_TYPE_BLOB: DUCKDB_TYPE = 18;
pub const DUCKDB_TYPE_DUCKDB_TYPE_DECIMAL: DUCKDB_TYPE = 19;
pub const DUCKDB_TYPE_DUCKDB_TYPE_TIMESTAMP_S: DUCKDB_TYPE = 20;
pub const DUCKDB_TYPE_DUCKDB_TYPE_TIMESTAMP_MS: DUCKDB_TYPE = 21;
pub const DUCKDB_TYPE_DUCKDB_TYPE_TIMESTAMP_NS: DUCKDB_TYPE = 22;
pub const DUCKDB_TYPE_DUCKDB_TYPE_ENUM: DUCKDB_TYPE = 23;
pub const DUCKDB_TYPE_DUCKDB_TYPE_LIST: DUCKDB_TYPE = 24;
pub const DUCKDB_TYPE_DUCKDB_TYPE_STRUCT: DUCKDB_TYPE = 25;
pub const DUCKDB_TYPE_DUCKDB_TYPE_MAP: DUCKDB_TYPE = 26;
pub const DUCKDB_TYPE_DUCKDB_TYPE_ARRAY: DUCKDB_TYPE = 33;
pub const DUCKDB_TYPE_DUCKDB_TYPE_UUID: DUCKDB_TYPE = 27;
pub const DUCKDB_TYPE_DUCKDB_TYPE_UNION: DUCKDB_TYPE = 28;
pub const DUCKDB_TYPE_DUCKDB_TYPE_BIT: DUCKDB_TYPE = 29;
pub const DUCKDB_TYPE_DUCKDB_TYPE_TIME_TZ: DUCKDB_TYPE = 30;
pub const DUCKDB_TYPE_DUCKDB_TYPE_TIMESTAMP_TZ: DUCKDB_TYPE = 31;
pub const DUCKDB_TYPE_DUCKDB_TYPE_ANY: DUCKDB_TYPE = 34;
pub const DUCKDB_TYPE_DUCKDB_TYPE_BIGNUM: DUCKDB_TYPE = 35;
pub const DUCKDB_TYPE_DUCKDB_TYPE_SQLNULL: DUCKDB_TYPE = 36;
pub const DUCKDB_TYPE_DUCKDB_TYPE_STRING_LITERAL: DUCKDB_TYPE = 37;
pub const DUCKDB_TYPE_DUCKDB_TYPE_INTEGER_LITERAL: DUCKDB_TYPE = 38;
pub const DUCKDB_TYPE_DUCKDB_TYPE_TIME_NS: DUCKDB_TYPE = 39;
pub const DUCKDB_TYPE_DUCKDB_TYPE_GEOMETRY: DUCKDB_TYPE = 40;
pub const DUCKDB_TYPE_DUCKDB_TYPE_VARIANT: DUCKDB_TYPE = 41;
pub type DUCKDB_TYPE = ::std::os::raw::c_uint;
pub use self::DUCKDB_TYPE as duckdb_type;
pub const duckdb_state_DuckDBSuccess: duckdb_state = 0;
pub const duckdb_state_DuckDBError: duckdb_state = 1;
pub type duckdb_state = ::std::os::raw::c_uint;
pub const duckdb_pending_state_DUCKDB_PENDING_RESULT_READY: duckdb_pending_state = 0;
pub const duckdb_pending_state_DUCKDB_PENDING_RESULT_NOT_READY: duckdb_pending_state = 1;
pub const duckdb_pending_state_DUCKDB_PENDING_ERROR: duckdb_pending_state = 2;
pub const duckdb_pending_state_DUCKDB_PENDING_NO_TASKS_AVAILABLE: duckdb_pending_state = 3;
pub type duckdb_pending_state = ::std::os::raw::c_uint;
pub const duckdb_result_type_DUCKDB_RESULT_TYPE_INVALID: duckdb_result_type = 0;
pub const duckdb_result_type_DUCKDB_RESULT_TYPE_CHANGED_ROWS: duckdb_result_type = 1;
pub const duckdb_result_type_DUCKDB_RESULT_TYPE_NOTHING: duckdb_result_type = 2;
pub const duckdb_result_type_DUCKDB_RESULT_TYPE_QUERY_RESULT: duckdb_result_type = 3;
pub type duckdb_result_type = ::std::os::raw::c_uint;
pub const duckdb_statement_type_DUCKDB_STATEMENT_TYPE_INVALID: duckdb_statement_type = 0;
pub const duckdb_statement_type_DUCKDB_STATEMENT_TYPE_SELECT: duckdb_statement_type = 1;
pub const duckdb_statement_type_DUCKDB_STATEMENT_TYPE_INSERT: duckdb_statement_type = 2;
pub const duckdb_statement_type_DUCKDB_STATEMENT_TYPE_UPDATE: duckdb_statement_type = 3;
pub const duckdb_statement_type_DUCKDB_STATEMENT_TYPE_EXPLAIN: duckdb_statement_type = 4;
pub const duckdb_statement_type_DUCKDB_STATEMENT_TYPE_DELETE: duckdb_statement_type = 5;
pub const duckdb_statement_type_DUCKDB_STATEMENT_TYPE_PREPARE: duckdb_statement_type = 6;
pub const duckdb_statement_type_DUCKDB_STATEMENT_TYPE_CREATE: duckdb_statement_type = 7;
pub const duckdb_statement_type_DUCKDB_STATEMENT_TYPE_EXECUTE: duckdb_statement_type = 8;
pub const duckdb_statement_type_DUCKDB_STATEMENT_TYPE_ALTER: duckdb_statement_type = 9;
pub const duckdb_statement_type_DUCKDB_STATEMENT_TYPE_TRANSACTION: duckdb_statement_type = 10;
pub const duckdb_statement_type_DUCKDB_STATEMENT_TYPE_COPY: duckdb_statement_type = 11;
pub const duckdb_statement_type_DUCKDB_STATEMENT_TYPE_ANALYZE: duckdb_statement_type = 12;
pub const duckdb_statement_type_DUCKDB_STATEMENT_TYPE_VARIABLE_SET: duckdb_statement_type = 13;
pub const duckdb_statement_type_DUCKDB_STATEMENT_TYPE_CREATE_FUNC: duckdb_statement_type = 14;
pub const duckdb_statement_type_DUCKDB_STATEMENT_TYPE_DROP: duckdb_statement_type = 15;
pub const duckdb_statement_type_DUCKDB_STATEMENT_TYPE_EXPORT: duckdb_statement_type = 16;
pub const duckdb_statement_type_DUCKDB_STATEMENT_TYPE_PRAGMA: duckdb_statement_type = 17;
pub const duckdb_statement_type_DUCKDB_STATEMENT_TYPE_VACUUM: duckdb_statement_type = 18;
pub const duckdb_statement_type_DUCKDB_STATEMENT_TYPE_CALL: duckdb_statement_type = 19;
pub const duckdb_statement_type_DUCKDB_STATEMENT_TYPE_SET: duckdb_statement_type = 20;
pub const duckdb_statement_type_DUCKDB_STATEMENT_TYPE_LOAD: duckdb_statement_type = 21;
pub const duckdb_statement_type_DUCKDB_STATEMENT_TYPE_RELATION: duckdb_statement_type = 22;
pub const duckdb_statement_type_DUCKDB_STATEMENT_TYPE_EXTENSION: duckdb_statement_type = 23;
pub const duckdb_statement_type_DUCKDB_STATEMENT_TYPE_LOGICAL_PLAN: duckdb_statement_type = 24;
pub const duckdb_statement_type_DUCKDB_STATEMENT_TYPE_ATTACH: duckdb_statement_type = 25;
pub const duckdb_statement_type_DUCKDB_STATEMENT_TYPE_DETACH: duckdb_statement_type = 26;
pub const duckdb_statement_type_DUCKDB_STATEMENT_TYPE_MULTI: duckdb_statement_type = 27;
pub type duckdb_statement_type = ::std::os::raw::c_uint;
pub const duckdb_error_type_DUCKDB_ERROR_INVALID: duckdb_error_type = 0;
pub const duckdb_error_type_DUCKDB_ERROR_OUT_OF_RANGE: duckdb_error_type = 1;
pub const duckdb_error_type_DUCKDB_ERROR_CONVERSION: duckdb_error_type = 2;
pub const duckdb_error_type_DUCKDB_ERROR_UNKNOWN_TYPE: duckdb_error_type = 3;
pub const duckdb_error_type_DUCKDB_ERROR_DECIMAL: duckdb_error_type = 4;
pub const duckdb_error_type_DUCKDB_ERROR_MISMATCH_TYPE: duckdb_error_type = 5;
pub const duckdb_error_type_DUCKDB_ERROR_DIVIDE_BY_ZERO: duckdb_error_type = 6;
pub const duckdb_error_type_DUCKDB_ERROR_OBJECT_SIZE: duckdb_error_type = 7;
pub const duckdb_error_type_DUCKDB_ERROR_INVALID_TYPE: duckdb_error_type = 8;
pub const duckdb_error_type_DUCKDB_ERROR_SERIALIZATION: duckdb_error_type = 9;
pub const duckdb_error_type_DUCKDB_ERROR_TRANSACTION: duckdb_error_type = 10;
pub const duckdb_error_type_DUCKDB_ERROR_NOT_IMPLEMENTED: duckdb_error_type = 11;
pub const duckdb_error_type_DUCKDB_ERROR_EXPRESSION: duckdb_error_type = 12;
pub const duckdb_error_type_DUCKDB_ERROR_CATALOG: duckdb_error_type = 13;
pub const duckdb_error_type_DUCKDB_ERROR_PARSER: duckdb_error_type = 14;
pub const duckdb_error_type_DUCKDB_ERROR_PLANNER: duckdb_error_type = 15;
pub const duckdb_error_type_DUCKDB_ERROR_SCHEDULER: duckdb_error_type = 16;
pub const duckdb_error_type_DUCKDB_ERROR_EXECUTOR: duckdb_error_type = 17;
pub const duckdb_error_type_DUCKDB_ERROR_CONSTRAINT: duckdb_error_type = 18;
pub const duckdb_error_type_DUCKDB_ERROR_INDEX: duckdb_error_type = 19;
pub const duckdb_error_type_DUCKDB_ERROR_STAT: duckdb_error_type = 20;
pub const duckdb_error_type_DUCKDB_ERROR_CONNECTION: duckdb_error_type = 21;
pub const duckdb_error_type_DUCKDB_ERROR_SYNTAX: duckdb_error_type = 22;
pub const duckdb_error_type_DUCKDB_ERROR_SETTINGS: duckdb_error_type = 23;
pub const duckdb_error_type_DUCKDB_ERROR_BINDER: duckdb_error_type = 24;
pub const duckdb_error_type_DUCKDB_ERROR_NETWORK: duckdb_error_type = 25;
pub const duckdb_error_type_DUCKDB_ERROR_OPTIMIZER: duckdb_error_type = 26;
pub const duckdb_error_type_DUCKDB_ERROR_NULL_POINTER: duckdb_error_type = 27;
pub const duckdb_error_type_DUCKDB_ERROR_IO: duckdb_error_type = 28;
pub const duckdb_error_type_DUCKDB_ERROR_INTERRUPT: duckdb_error_type = 29;
pub const duckdb_error_type_DUCKDB_ERROR_FATAL: duckdb_error_type = 30;
pub const duckdb_error_type_DUCKDB_ERROR_INTERNAL: duckdb_error_type = 31;
pub const duckdb_error_type_DUCKDB_ERROR_INVALID_INPUT: duckdb_error_type = 32;
pub const duckdb_error_type_DUCKDB_ERROR_OUT_OF_MEMORY: duckdb_error_type = 33;
pub const duckdb_error_type_DUCKDB_ERROR_PERMISSION: duckdb_error_type = 34;
pub const duckdb_error_type_DUCKDB_ERROR_PARAMETER_NOT_RESOLVED: duckdb_error_type = 35;
pub const duckdb_error_type_DUCKDB_ERROR_PARAMETER_NOT_ALLOWED: duckdb_error_type = 36;
pub const duckdb_error_type_DUCKDB_ERROR_DEPENDENCY: duckdb_error_type = 37;
pub const duckdb_error_type_DUCKDB_ERROR_HTTP: duckdb_error_type = 38;
pub const duckdb_error_type_DUCKDB_ERROR_MISSING_EXTENSION: duckdb_error_type = 39;
pub const duckdb_error_type_DUCKDB_ERROR_AUTOLOAD: duckdb_error_type = 40;
pub const duckdb_error_type_DUCKDB_ERROR_SEQUENCE: duckdb_error_type = 41;
pub const duckdb_error_type_DUCKDB_INVALID_CONFIGURATION: duckdb_error_type = 42;
pub type duckdb_error_type = ::std::os::raw::c_uint;
pub const duckdb_cast_mode_DUCKDB_CAST_NORMAL: duckdb_cast_mode = 0;
pub const duckdb_cast_mode_DUCKDB_CAST_TRY: duckdb_cast_mode = 1;
pub type duckdb_cast_mode = ::std::os::raw::c_uint;
pub const duckdb_file_flag_DUCKDB_FILE_FLAG_INVALID: duckdb_file_flag = 0;
pub const duckdb_file_flag_DUCKDB_FILE_FLAG_READ: duckdb_file_flag = 1;
pub const duckdb_file_flag_DUCKDB_FILE_FLAG_WRITE: duckdb_file_flag = 2;
pub const duckdb_file_flag_DUCKDB_FILE_FLAG_CREATE: duckdb_file_flag = 3;
pub const duckdb_file_flag_DUCKDB_FILE_FLAG_CREATE_NEW: duckdb_file_flag = 4;
pub const duckdb_file_flag_DUCKDB_FILE_FLAG_APPEND: duckdb_file_flag = 5;
pub type duckdb_file_flag = ::std::os::raw::c_uint;
pub const duckdb_config_option_scope_DUCKDB_CONFIG_OPTION_SCOPE_INVALID: duckdb_config_option_scope = 0;
pub const duckdb_config_option_scope_DUCKDB_CONFIG_OPTION_SCOPE_LOCAL: duckdb_config_option_scope = 1;
pub const duckdb_config_option_scope_DUCKDB_CONFIG_OPTION_SCOPE_SESSION: duckdb_config_option_scope = 2;
pub const duckdb_config_option_scope_DUCKDB_CONFIG_OPTION_SCOPE_GLOBAL: duckdb_config_option_scope = 3;
pub type duckdb_config_option_scope = ::std::os::raw::c_uint;
pub const duckdb_catalog_entry_type_DUCKDB_CATALOG_ENTRY_TYPE_INVALID: duckdb_catalog_entry_type = 0;
pub const duckdb_catalog_entry_type_DUCKDB_CATALOG_ENTRY_TYPE_TABLE: duckdb_catalog_entry_type = 1;
pub const duckdb_catalog_entry_type_DUCKDB_CATALOG_ENTRY_TYPE_SCHEMA: duckdb_catalog_entry_type = 2;
pub const duckdb_catalog_entry_type_DUCKDB_CATALOG_ENTRY_TYPE_VIEW: duckdb_catalog_entry_type = 3;
pub const duckdb_catalog_entry_type_DUCKDB_CATALOG_ENTRY_TYPE_INDEX: duckdb_catalog_entry_type = 4;
pub const duckdb_catalog_entry_type_DUCKDB_CATALOG_ENTRY_TYPE_PREPARED_STATEMENT: duckdb_catalog_entry_type = 5;
pub const duckdb_catalog_entry_type_DUCKDB_CATALOG_ENTRY_TYPE_SEQUENCE: duckdb_catalog_entry_type = 6;
pub const duckdb_catalog_entry_type_DUCKDB_CATALOG_ENTRY_TYPE_COLLATION: duckdb_catalog_entry_type = 7;
pub const duckdb_catalog_entry_type_DUCKDB_CATALOG_ENTRY_TYPE_TYPE: duckdb_catalog_entry_type = 8;
pub const duckdb_catalog_entry_type_DUCKDB_CATALOG_ENTRY_TYPE_DATABASE: duckdb_catalog_entry_type = 9;
pub type duckdb_catalog_entry_type = ::std::os::raw::c_uint;
pub type idx_t = u64;
pub type sel_t = u32;
pub type duckdb_delete_callback_t = ::std::option::Option<unsafe extern "C" fn(data: *mut ::std::os::raw::c_void)>;
pub type duckdb_copy_callback_t =
    ::std::option::Option<unsafe extern "C" fn(data: *mut ::std::os::raw::c_void) -> *mut ::std::os::raw::c_void>;
pub type duckdb_task_state = *mut ::std::os::raw::c_void;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct duckdb_date {
    pub days: i32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct duckdb_date_struct {
    pub year: i32,
    pub month: i8,
    pub day: i8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct duckdb_time {
    pub micros: i64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct duckdb_time_struct {
    pub hour: i8,
    pub min: i8,
    pub sec: i8,
    pub micros: i32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct duckdb_time_ns {
    pub nanos: i64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct duckdb_time_tz {
    pub bits: u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct duckdb_time_tz_struct {
    pub time: duckdb_time_struct,
    pub offset: i32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct duckdb_timestamp {
    pub micros: i64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct duckdb_timestamp_struct {
    pub date: duckdb_date_struct,
    pub time: duckdb_time_struct,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct duckdb_timestamp_s {
    pub seconds: i64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct duckdb_timestamp_ms {
    pub millis: i64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct duckdb_timestamp_ns {
    pub nanos: i64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct duckdb_interval {
    pub months: i32,
    pub days: i32,
    pub micros: i64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct duckdb_hugeint {
    pub lower: u64,
    pub upper: i64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct duckdb_uhugeint {
    pub lower: u64,
    pub upper: u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct duckdb_decimal {
    pub width: u8,
    pub scale: u8,
    pub value: duckdb_hugeint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct duckdb_query_progress_type {
    pub percentage: f64,
    pub rows_processed: u64,
    pub total_rows_to_process: u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct duckdb_string_t {
    pub value: duckdb_string_t__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union duckdb_string_t__bindgen_ty_1 {
    pub pointer: duckdb_string_t__bindgen_ty_1__bindgen_ty_1,
    pub inlined: duckdb_string_t__bindgen_ty_1__bindgen_ty_2,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct duckdb_string_t__bindgen_ty_1__bindgen_ty_1 {
    pub length: u32,
    pub prefix: [::std::os::raw::c_char; 4usize],
    pub ptr: *mut ::std::os::raw::c_char,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct duckdb_string_t__bindgen_ty_1__bindgen_ty_2 {
    pub length: u32,
    pub inlined: [::std::os::raw::c_char; 12usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct duckdb_list_entry {
    pub offset: u64,
    pub length: u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct duckdb_column {
    pub deprecated_data: *mut ::std::os::raw::c_void,
    pub deprecated_nullmask: *mut bool,
    pub deprecated_type: duckdb_type,
    pub deprecated_name: *mut ::std::os::raw::c_char,
    pub internal_data: *mut ::std::os::raw::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _duckdb_vector {
    pub internal_ptr: *mut ::std::os::raw::c_void,
}
pub type duckdb_vector = *mut _duckdb_vector;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _duckdb_selection_vector {
    pub internal_ptr: *mut ::std::os::raw::c_void,
}
pub type duckdb_selection_vector = *mut _duckdb_selection_vector;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct duckdb_string {
    pub data: *mut ::std::os::raw::c_char,
    pub size: idx_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct duckdb_blob {
    pub data: *mut ::std::os::raw::c_void,
    pub size: idx_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct duckdb_bit {
    pub data: *mut u8,
    pub size: idx_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct duckdb_bignum {
    pub data: *mut u8,
    pub size: idx_t,
    pub is_negative: bool,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct duckdb_result {
    pub deprecated_column_count: idx_t,
    pub deprecated_row_count: idx_t,
    pub deprecated_rows_changed: idx_t,
    pub deprecated_columns: *mut duckdb_column,
    pub deprecated_error_message: *mut ::std::os::raw::c_char,
    pub internal_data: *mut ::std::os::raw::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _duckdb_instance_cache {
    pub internal_ptr: *mut ::std::os::raw::c_void,
}
pub type duckdb_instance_cache = *mut _duckdb_instance_cache;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _duckdb_database {
    pub internal_ptr: *mut ::std::os::raw::c_void,
}
pub type duckdb_database = *mut _duckdb_database;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _duckdb_connection {
    pub internal_ptr: *mut ::std::os::raw::c_void,
}
pub type duckdb_connection = *mut _duckdb_connection;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _duckdb_client_context {
    pub internal_ptr: *mut ::std::os::raw::c_void,
}
pub type duckdb_client_context = *mut _duckdb_client_context;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _duckdb_prepared_statement {
    pub internal_ptr: *mut ::std::os::raw::c_void,
}
pub type duckdb_prepared_statement = *mut _duckdb_prepared_statement;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _duckdb_extracted_statements {
    pub internal_ptr: *mut ::std::os::raw::c_void,
}
pub type duckdb_extracted_statements = *mut _duckdb_extracted_statements;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _duckdb_pending_result {
    pub internal_ptr: *mut ::std::os::raw::c_void,
}
pub type duckdb_pending_result = *mut _duckdb_pending_result;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _duckdb_appender {
    pub internal_ptr: *mut ::std::os::raw::c_void,
}
pub type duckdb_appender = *mut _duckdb_appender;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _duckdb_table_description {
    pub internal_ptr: *mut ::std::os::raw::c_void,
}
pub type duckdb_table_description = *mut _duckdb_table_description;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _duckdb_config {
    pub internal_ptr: *mut ::std::os::raw::c_void,
}
pub type duckdb_config = *mut _duckdb_config;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _duckdb_config_option {
    pub internal_ptr: *mut ::std::os::raw::c_void,
}
pub type duckdb_config_option = *mut _duckdb_config_option;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _duckdb_logical_type {
    pub internal_ptr: *mut ::std::os::raw::c_void,
}
pub type duckdb_logical_type = *mut _duckdb_logical_type;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _duckdb_create_type_info {
    pub internal_ptr: *mut ::std::os::raw::c_void,
}
pub type duckdb_create_type_info = *mut _duckdb_create_type_info;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _duckdb_data_chunk {
    pub internal_ptr: *mut ::std::os::raw::c_void,
}
pub type duckdb_data_chunk = *mut _duckdb_data_chunk;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _duckdb_value {
    pub internal_ptr: *mut ::std::os::raw::c_void,
}
pub type duckdb_value = *mut _duckdb_value;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _duckdb_profiling_info {
    pub internal_ptr: *mut ::std::os::raw::c_void,
}
pub type duckdb_profiling_info = *mut _duckdb_profiling_info;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _duckdb_error_data {
    pub internal_ptr: *mut ::std::os::raw::c_void,
}
pub type duckdb_error_data = *mut _duckdb_error_data;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _duckdb_expression {
    pub internal_ptr: *mut ::std::os::raw::c_void,
}
pub type duckdb_expression = *mut _duckdb_expression;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _duckdb_extension_info {
    pub internal_ptr: *mut ::std::os::raw::c_void,
}
pub type duckdb_extension_info = *mut _duckdb_extension_info;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _duckdb_function_info {
    pub internal_ptr: *mut ::std::os::raw::c_void,
}
pub type duckdb_function_info = *mut _duckdb_function_info;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _duckdb_bind_info {
    pub internal_ptr: *mut ::std::os::raw::c_void,
}
pub type duckdb_bind_info = *mut _duckdb_bind_info;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _duckdb_init_info {
    pub internal_ptr: *mut ::std::os::raw::c_void,
}
pub type duckdb_init_info = *mut _duckdb_init_info;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _duckdb_scalar_function {
    pub internal_ptr: *mut ::std::os::raw::c_void,
}
pub type duckdb_scalar_function = *mut _duckdb_scalar_function;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _duckdb_scalar_function_set {
    pub internal_ptr: *mut ::std::os::raw::c_void,
}
pub type duckdb_scalar_function_set = *mut _duckdb_scalar_function_set;
pub type duckdb_scalar_function_bind_t = ::std::option::Option<unsafe extern "C" fn(info: duckdb_bind_info)>;
pub type duckdb_scalar_function_init_t = ::std::option::Option<unsafe extern "C" fn(info: duckdb_init_info)>;
pub type duckdb_scalar_function_t = ::std::option::Option<
    unsafe extern "C" fn(info: duckdb_function_info, input: duckdb_data_chunk, output: duckdb_vector),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _duckdb_aggregate_function {
    pub internal_ptr: *mut ::std::os::raw::c_void,
}
pub type duckdb_aggregate_function = *mut _duckdb_aggregate_function;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _duckdb_aggregate_function_set {
    pub internal_ptr: *mut ::std::os::raw::c_void,
}
pub type duckdb_aggregate_function_set = *mut _duckdb_aggregate_function_set;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _duckdb_aggregate_state {
    pub internal_ptr: *mut ::std::os::raw::c_void,
}
pub type duckdb_aggregate_state = *mut _duckdb_aggregate_state;
pub type duckdb_aggregate_state_size = ::std::option::Option<unsafe extern "C" fn(info: duckdb_function_info) -> idx_t>;
pub type duckdb_aggregate_init_t =
    ::std::option::Option<unsafe extern "C" fn(info: duckdb_function_info, state: duckdb_aggregate_state)>;
pub type duckdb_aggregate_destroy_t =
    ::std::option::Option<unsafe extern "C" fn(states: *mut duckdb_aggregate_state, count: idx_t)>;
pub type duckdb_aggregate_update_t = ::std::option::Option<
    unsafe extern "C" fn(info: duckdb_function_info, input: duckdb_data_chunk, states: *mut duckdb_aggregate_state),
>;
pub type duckdb_aggregate_combine_t = ::std::option::Option<
    unsafe extern "C" fn(
        info: duckdb_function_info,
        source: *mut duckdb_aggregate_state,
        target: *mut duckdb_aggregate_state,
        count: idx_t,
    ),
>;
pub type duckdb_aggregate_finalize_t = ::std::option::Option<
    unsafe extern "C" fn(
        info: duckdb_function_info,
        source: *mut duckdb_aggregate_state,
        result: duckdb_vector,
        count: idx_t,
        offset: idx_t,
    ),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _duckdb_table_function {
    pub internal_ptr: *mut ::std::os::raw::c_void,
}
pub type duckdb_table_function = *mut _duckdb_table_function;
pub type duckdb_table_function_bind_t = ::std::option::Option<unsafe extern "C" fn(info: duckdb_bind_info)>;
pub type duckdb_table_function_init_t = ::std::option::Option<unsafe extern "C" fn(info: duckdb_init_info)>;
pub type duckdb_table_function_t =
    ::std::option::Option<unsafe extern "C" fn(info: duckdb_function_info, output: duckdb_data_chunk)>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _duckdb_copy_function {
    pub internal_ptr: *mut ::std::os::raw::c_void,
}
pub type duckdb_copy_function = *mut _duckdb_copy_function;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _duckdb_copy_function_bind_info {
    pub internal_ptr: *mut ::std::os::raw::c_void,
}
pub type duckdb_copy_function_bind_info = *mut _duckdb_copy_function_bind_info;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _duckdb_copy_function_global_init_info {
    pub internal_ptr: *mut ::std::os::raw::c_void,
}
pub type duckdb_copy_function_global_init_info = *mut _duckdb_copy_function_global_init_info;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _duckdb_copy_function_sink_info {
    pub internal_ptr: *mut ::std::os::raw::c_void,
}
pub type duckdb_copy_function_sink_info = *mut _duckdb_copy_function_sink_info;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _duckdb_copy_function_finalize_info {
    pub internal_ptr: *mut ::std::os::raw::c_void,
}
pub type duckdb_copy_function_finalize_info = *mut _duckdb_copy_function_finalize_info;
pub type duckdb_copy_function_bind_t =
    ::std::option::Option<unsafe extern "C" fn(info: duckdb_copy_function_bind_info)>;
pub type duckdb_copy_function_global_init_t =
    ::std::option::Option<unsafe extern "C" fn(info: duckdb_copy_function_global_init_info)>;
pub type duckdb_copy_function_sink_t =
    ::std::option::Option<unsafe extern "C" fn(info: duckdb_copy_function_sink_info, input: duckdb_data_chunk)>;
pub type duckdb_copy_function_finalize_t =
    ::std::option::Option<unsafe extern "C" fn(info: duckdb_copy_function_finalize_info)>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _duckdb_cast_function {
    pub internal_ptr: *mut ::std::os::raw::c_void,
}
pub type duckdb_cast_function = *mut _duckdb_cast_function;
pub type duckdb_cast_function_t = ::std::option::Option<
    unsafe extern "C" fn(info: duckdb_function_info, count: idx_t, input: duckdb_vector, output: duckdb_vector) -> bool,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _duckdb_replacement_scan_info {
    pub internal_ptr: *mut ::std::os::raw::c_void,
}
pub type duckdb_replacement_scan_info = *mut _duckdb_replacement_scan_info;
pub type duckdb_replacement_callback_t = ::std::option::Option<
    unsafe extern "C" fn(
        info: duckdb_replacement_scan_info,
        table_name: *const ::std::os::raw::c_char,
        data: *mut ::std::os::raw::c_void,
    ),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _duckdb_arrow {
    pub internal_ptr: *mut ::std::os::raw::c_void,
}
pub type duckdb_arrow = *mut _duckdb_arrow;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _duckdb_arrow_stream {
    pub internal_ptr: *mut ::std::os::raw::c_void,
}
pub type duckdb_arrow_stream = *mut _duckdb_arrow_stream;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _duckdb_arrow_schema {
    pub internal_ptr: *mut ::std::os::raw::c_void,
}
pub type duckdb_arrow_schema = *mut _duckdb_arrow_schema;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _duckdb_arrow_converted_schema {
    pub internal_ptr: *mut ::std::os::raw::c_void,
}
pub type duckdb_arrow_converted_schema = *mut _duckdb_arrow_converted_schema;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _duckdb_arrow_array {
    pub internal_ptr: *mut ::std::os::raw::c_void,
}
pub type duckdb_arrow_array = *mut _duckdb_arrow_array;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _duckdb_arrow_options {
    pub internal_ptr: *mut ::std::os::raw::c_void,
}
pub type duckdb_arrow_options = *mut _duckdb_arrow_options;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _duckdb_file_open_options {
    pub internal_ptr: *mut ::std::os::raw::c_void,
}
pub type duckdb_file_open_options = *mut _duckdb_file_open_options;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _duckdb_file_system {
    pub internal_ptr: *mut ::std::os::raw::c_void,
}
pub type duckdb_file_system = *mut _duckdb_file_system;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _duckdb_file_handle {
    pub internal_ptr: *mut ::std::os::raw::c_void,
}
pub type duckdb_file_handle = *mut _duckdb_file_handle;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _duckdb_catalog {
    pub internal_ptr: *mut ::std::os::raw::c_void,
}
pub type duckdb_catalog = *mut _duckdb_catalog;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _duckdb_catalog_entry {
    pub internal_ptr: *mut ::std::os::raw::c_void,
}
pub type duckdb_catalog_entry = *mut _duckdb_catalog_entry;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _duckdb_log_storage {
    pub internal_ptr: *mut ::std::os::raw::c_void,
}
pub type duckdb_log_storage = *mut _duckdb_log_storage;
pub type duckdb_logger_write_log_entry_t = ::std::option::Option<
    unsafe extern "C" fn(
        extra_data: *mut ::std::os::raw::c_void,
        timestamp: *mut duckdb_timestamp,
        level: *const ::std::os::raw::c_char,
        log_type: *const ::std::os::raw::c_char,
        log_message: *const ::std::os::raw::c_char,
    ),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct duckdb_extension_access {
    pub set_error:
        ::std::option::Option<unsafe extern "C" fn(info: duckdb_extension_info, error: *const ::std::os::raw::c_char)>,
    pub get_database: ::std::option::Option<unsafe extern "C" fn(info: duckdb_extension_info) -> *mut duckdb_database>,
    pub get_api: ::std::option::Option<
        unsafe extern "C" fn(
            info: duckdb_extension_info,
            version: *const ::std::os::raw::c_char,
        ) -> *const ::std::os::raw::c_void,
    >,
}
unsafe extern "C" {
    pub fn duckdb_create_instance_cache() -> duckdb_instance_cache;
}
unsafe extern "C" {
    pub fn duckdb_get_or_create_from_cache(
        instance_cache: duckdb_instance_cache,
        path: *const ::std::os::raw::c_char,
        out_database: *mut duckdb_database,
        config: duckdb_config,
        out_error: *mut *mut ::std::os::raw::c_char,
    ) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_destroy_instance_cache(instance_cache: *mut duckdb_instance_cache);
}
unsafe extern "C" {
    pub fn duckdb_open(path: *const ::std::os::raw::c_char, out_database: *mut duckdb_database) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_open_ext(
        path: *const ::std::os::raw::c_char,
        out_database: *mut duckdb_database,
        config: duckdb_config,
        out_error: *mut *mut ::std::os::raw::c_char,
    ) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_close(database: *mut duckdb_database);
}
unsafe extern "C" {
    pub fn duckdb_connect(database: duckdb_database, out_connection: *mut duckdb_connection) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_interrupt(connection: duckdb_connection);
}
unsafe extern "C" {
    pub fn duckdb_query_progress(connection: duckdb_connection) -> duckdb_query_progress_type;
}
unsafe extern "C" {
    pub fn duckdb_disconnect(connection: *mut duckdb_connection);
}
unsafe extern "C" {
    pub fn duckdb_connection_get_client_context(connection: duckdb_connection, out_context: *mut duckdb_client_context);
}
unsafe extern "C" {
    pub fn duckdb_connection_get_arrow_options(
        connection: duckdb_connection,
        out_arrow_options: *mut duckdb_arrow_options,
    );
}
unsafe extern "C" {
    pub fn duckdb_client_context_get_connection_id(context: duckdb_client_context) -> idx_t;
}
unsafe extern "C" {
    pub fn duckdb_destroy_client_context(context: *mut duckdb_client_context);
}
unsafe extern "C" {
    pub fn duckdb_destroy_arrow_options(arrow_options: *mut duckdb_arrow_options);
}
unsafe extern "C" {
    pub fn duckdb_library_version() -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn duckdb_get_table_names(
        connection: duckdb_connection,
        query: *const ::std::os::raw::c_char,
        qualified: bool,
    ) -> duckdb_value;
}
unsafe extern "C" {
    pub fn duckdb_create_config(out_config: *mut duckdb_config) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_config_count() -> usize;
}
unsafe extern "C" {
    pub fn duckdb_get_config_flag(
        index: usize,
        out_name: *mut *const ::std::os::raw::c_char,
        out_description: *mut *const ::std::os::raw::c_char,
    ) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_set_config(
        config: duckdb_config,
        name: *const ::std::os::raw::c_char,
        option: *const ::std::os::raw::c_char,
    ) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_destroy_config(config: *mut duckdb_config);
}
unsafe extern "C" {
    pub fn duckdb_create_error_data(
        type_: duckdb_error_type,
        message: *const ::std::os::raw::c_char,
    ) -> duckdb_error_data;
}
unsafe extern "C" {
    pub fn duckdb_destroy_error_data(error_data: *mut duckdb_error_data);
}
unsafe extern "C" {
    pub fn duckdb_error_data_error_type(error_data: duckdb_error_data) -> duckdb_error_type;
}
unsafe extern "C" {
    pub fn duckdb_error_data_message(error_data: duckdb_error_data) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn duckdb_error_data_has_error(error_data: duckdb_error_data) -> bool;
}
unsafe extern "C" {
    pub fn duckdb_query(
        connection: duckdb_connection,
        query: *const ::std::os::raw::c_char,
        out_result: *mut duckdb_result,
    ) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_destroy_result(result: *mut duckdb_result);
}
unsafe extern "C" {
    pub fn duckdb_column_name(result: *mut duckdb_result, col: idx_t) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn duckdb_column_type(result: *mut duckdb_result, col: idx_t) -> duckdb_type;
}
unsafe extern "C" {
    pub fn duckdb_result_statement_type(result: duckdb_result) -> duckdb_statement_type;
}
unsafe extern "C" {
    pub fn duckdb_column_logical_type(result: *mut duckdb_result, col: idx_t) -> duckdb_logical_type;
}
unsafe extern "C" {
    pub fn duckdb_result_get_arrow_options(result: *mut duckdb_result) -> duckdb_arrow_options;
}
unsafe extern "C" {
    pub fn duckdb_column_count(result: *mut duckdb_result) -> idx_t;
}
unsafe extern "C" {
    pub fn duckdb_row_count(result: *mut duckdb_result) -> idx_t;
}
unsafe extern "C" {
    pub fn duckdb_rows_changed(result: *mut duckdb_result) -> idx_t;
}
unsafe extern "C" {
    pub fn duckdb_column_data(result: *mut duckdb_result, col: idx_t) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn duckdb_nullmask_data(result: *mut duckdb_result, col: idx_t) -> *mut bool;
}
unsafe extern "C" {
    pub fn duckdb_result_error(result: *mut duckdb_result) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn duckdb_result_error_type(result: *mut duckdb_result) -> duckdb_error_type;
}
unsafe extern "C" {
    pub fn duckdb_result_get_chunk(result: duckdb_result, chunk_index: idx_t) -> duckdb_data_chunk;
}
unsafe extern "C" {
    pub fn duckdb_result_is_streaming(result: duckdb_result) -> bool;
}
unsafe extern "C" {
    pub fn duckdb_result_chunk_count(result: duckdb_result) -> idx_t;
}
unsafe extern "C" {
    pub fn duckdb_result_return_type(result: duckdb_result) -> duckdb_result_type;
}
unsafe extern "C" {
    pub fn duckdb_value_boolean(result: *mut duckdb_result, col: idx_t, row: idx_t) -> bool;
}
unsafe extern "C" {
    pub fn duckdb_value_int8(result: *mut duckdb_result, col: idx_t, row: idx_t) -> i8;
}
unsafe extern "C" {
    pub fn duckdb_value_int16(result: *mut duckdb_result, col: idx_t, row: idx_t) -> i16;
}
unsafe extern "C" {
    pub fn duckdb_value_int32(result: *mut duckdb_result, col: idx_t, row: idx_t) -> i32;
}
unsafe extern "C" {
    pub fn duckdb_value_int64(result: *mut duckdb_result, col: idx_t, row: idx_t) -> i64;
}
unsafe extern "C" {
    pub fn duckdb_value_hugeint(result: *mut duckdb_result, col: idx_t, row: idx_t) -> duckdb_hugeint;
}
unsafe extern "C" {
    pub fn duckdb_value_uhugeint(result: *mut duckdb_result, col: idx_t, row: idx_t) -> duckdb_uhugeint;
}
unsafe extern "C" {
    pub fn duckdb_value_decimal(result: *mut duckdb_result, col: idx_t, row: idx_t) -> duckdb_decimal;
}
unsafe extern "C" {
    pub fn duckdb_value_uint8(result: *mut duckdb_result, col: idx_t, row: idx_t) -> u8;
}
unsafe extern "C" {
    pub fn duckdb_value_uint16(result: *mut duckdb_result, col: idx_t, row: idx_t) -> u16;
}
unsafe extern "C" {
    pub fn duckdb_value_uint32(result: *mut duckdb_result, col: idx_t, row: idx_t) -> u32;
}
unsafe extern "C" {
    pub fn duckdb_value_uint64(result: *mut duckdb_result, col: idx_t, row: idx_t) -> u64;
}
unsafe extern "C" {
    pub fn duckdb_value_float(result: *mut duckdb_result, col: idx_t, row: idx_t) -> f32;
}
unsafe extern "C" {
    pub fn duckdb_value_double(result: *mut duckdb_result, col: idx_t, row: idx_t) -> f64;
}
unsafe extern "C" {
    pub fn duckdb_value_date(result: *mut duckdb_result, col: idx_t, row: idx_t) -> duckdb_date;
}
unsafe extern "C" {
    pub fn duckdb_value_time(result: *mut duckdb_result, col: idx_t, row: idx_t) -> duckdb_time;
}
unsafe extern "C" {
    pub fn duckdb_value_timestamp(result: *mut duckdb_result, col: idx_t, row: idx_t) -> duckdb_timestamp;
}
unsafe extern "C" {
    pub fn duckdb_value_interval(result: *mut duckdb_result, col: idx_t, row: idx_t) -> duckdb_interval;
}
unsafe extern "C" {
    pub fn duckdb_value_varchar(result: *mut duckdb_result, col: idx_t, row: idx_t) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn duckdb_value_string(result: *mut duckdb_result, col: idx_t, row: idx_t) -> duckdb_string;
}
unsafe extern "C" {
    pub fn duckdb_value_varchar_internal(
        result: *mut duckdb_result,
        col: idx_t,
        row: idx_t,
    ) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn duckdb_value_string_internal(result: *mut duckdb_result, col: idx_t, row: idx_t) -> duckdb_string;
}
unsafe extern "C" {
    pub fn duckdb_value_blob(result: *mut duckdb_result, col: idx_t, row: idx_t) -> duckdb_blob;
}
unsafe extern "C" {
    pub fn duckdb_value_is_null(result: *mut duckdb_result, col: idx_t, row: idx_t) -> bool;
}
unsafe extern "C" {
    pub fn duckdb_malloc(size: usize) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn duckdb_free(ptr: *mut ::std::os::raw::c_void);
}
unsafe extern "C" {
    pub fn duckdb_vector_size() -> idx_t;
}
unsafe extern "C" {
    pub fn duckdb_string_is_inlined(string: duckdb_string_t) -> bool;
}
unsafe extern "C" {
    pub fn duckdb_string_t_length(string: duckdb_string_t) -> u32;
}
unsafe extern "C" {
    pub fn duckdb_string_t_data(string: *mut duckdb_string_t) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn duckdb_valid_utf8_check(str_: *const ::std::os::raw::c_char, len: idx_t) -> duckdb_error_data;
}
unsafe extern "C" {
    pub fn duckdb_from_date(date: duckdb_date) -> duckdb_date_struct;
}
unsafe extern "C" {
    pub fn duckdb_to_date(date: duckdb_date_struct) -> duckdb_date;
}
unsafe extern "C" {
    pub fn duckdb_is_finite_date(date: duckdb_date) -> bool;
}
unsafe extern "C" {
    pub fn duckdb_from_time(time: duckdb_time) -> duckdb_time_struct;
}
unsafe extern "C" {
    pub fn duckdb_create_time_tz(micros: i64, offset: i32) -> duckdb_time_tz;
}
unsafe extern "C" {
    pub fn duckdb_from_time_tz(micros: duckdb_time_tz) -> duckdb_time_tz_struct;
}
unsafe extern "C" {
    pub fn duckdb_to_time(time: duckdb_time_struct) -> duckdb_time;
}
unsafe extern "C" {
    pub fn duckdb_from_timestamp(ts: duckdb_timestamp) -> duckdb_timestamp_struct;
}
unsafe extern "C" {
    pub fn duckdb_to_timestamp(ts: duckdb_timestamp_struct) -> duckdb_timestamp;
}
unsafe extern "C" {
    pub fn duckdb_is_finite_timestamp(ts: duckdb_timestamp) -> bool;
}
unsafe extern "C" {
    pub fn duckdb_is_finite_timestamp_s(ts: duckdb_timestamp_s) -> bool;
}
unsafe extern "C" {
    pub fn duckdb_is_finite_timestamp_ms(ts: duckdb_timestamp_ms) -> bool;
}
unsafe extern "C" {
    pub fn duckdb_is_finite_timestamp_ns(ts: duckdb_timestamp_ns) -> bool;
}
unsafe extern "C" {
    pub fn duckdb_hugeint_to_double(val: duckdb_hugeint) -> f64;
}
unsafe extern "C" {
    pub fn duckdb_double_to_hugeint(val: f64) -> duckdb_hugeint;
}
unsafe extern "C" {
    pub fn duckdb_uhugeint_to_double(val: duckdb_uhugeint) -> f64;
}
unsafe extern "C" {
    pub fn duckdb_double_to_uhugeint(val: f64) -> duckdb_uhugeint;
}
unsafe extern "C" {
    pub fn duckdb_double_to_decimal(val: f64, width: u8, scale: u8) -> duckdb_decimal;
}
unsafe extern "C" {
    pub fn duckdb_decimal_to_double(val: duckdb_decimal) -> f64;
}
unsafe extern "C" {
    pub fn duckdb_prepare(
        connection: duckdb_connection,
        query: *const ::std::os::raw::c_char,
        out_prepared_statement: *mut duckdb_prepared_statement,
    ) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_destroy_prepare(prepared_statement: *mut duckdb_prepared_statement);
}
unsafe extern "C" {
    pub fn duckdb_prepare_error(prepared_statement: duckdb_prepared_statement) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn duckdb_nparams(prepared_statement: duckdb_prepared_statement) -> idx_t;
}
unsafe extern "C" {
    pub fn duckdb_parameter_name(
        prepared_statement: duckdb_prepared_statement,
        index: idx_t,
    ) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn duckdb_param_type(prepared_statement: duckdb_prepared_statement, param_idx: idx_t) -> duckdb_type;
}
unsafe extern "C" {
    pub fn duckdb_param_logical_type(
        prepared_statement: duckdb_prepared_statement,
        param_idx: idx_t,
    ) -> duckdb_logical_type;
}
unsafe extern "C" {
    pub fn duckdb_clear_bindings(prepared_statement: duckdb_prepared_statement) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_prepared_statement_type(statement: duckdb_prepared_statement) -> duckdb_statement_type;
}
unsafe extern "C" {
    pub fn duckdb_prepared_statement_column_count(prepared_statement: duckdb_prepared_statement) -> idx_t;
}
unsafe extern "C" {
    pub fn duckdb_prepared_statement_column_name(
        prepared_statement: duckdb_prepared_statement,
        col_idx: idx_t,
    ) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn duckdb_prepared_statement_column_logical_type(
        prepared_statement: duckdb_prepared_statement,
        col_idx: idx_t,
    ) -> duckdb_logical_type;
}
unsafe extern "C" {
    pub fn duckdb_prepared_statement_column_type(
        prepared_statement: duckdb_prepared_statement,
        col_idx: idx_t,
    ) -> duckdb_type;
}
unsafe extern "C" {
    pub fn duckdb_bind_value(
        prepared_statement: duckdb_prepared_statement,
        param_idx: idx_t,
        val: duckdb_value,
    ) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_bind_parameter_index(
        prepared_statement: duckdb_prepared_statement,
        param_idx_out: *mut idx_t,
        name: *const ::std::os::raw::c_char,
    ) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_bind_boolean(
        prepared_statement: duckdb_prepared_statement,
        param_idx: idx_t,
        val: bool,
    ) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_bind_int8(prepared_statement: duckdb_prepared_statement, param_idx: idx_t, val: i8) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_bind_int16(prepared_statement: duckdb_prepared_statement, param_idx: idx_t, val: i16)
    -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_bind_int32(prepared_statement: duckdb_prepared_statement, param_idx: idx_t, val: i32)
    -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_bind_int64(prepared_statement: duckdb_prepared_statement, param_idx: idx_t, val: i64)
    -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_bind_hugeint(
        prepared_statement: duckdb_prepared_statement,
        param_idx: idx_t,
        val: duckdb_hugeint,
    ) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_bind_uhugeint(
        prepared_statement: duckdb_prepared_statement,
        param_idx: idx_t,
        val: duckdb_uhugeint,
    ) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_bind_decimal(
        prepared_statement: duckdb_prepared_statement,
        param_idx: idx_t,
        val: duckdb_decimal,
    ) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_bind_uint8(prepared_statement: duckdb_prepared_statement, param_idx: idx_t, val: u8) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_bind_uint16(
        prepared_statement: duckdb_prepared_statement,
        param_idx: idx_t,
        val: u16,
    ) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_bind_uint32(
        prepared_statement: duckdb_prepared_statement,
        param_idx: idx_t,
        val: u32,
    ) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_bind_uint64(
        prepared_statement: duckdb_prepared_statement,
        param_idx: idx_t,
        val: u64,
    ) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_bind_float(prepared_statement: duckdb_prepared_statement, param_idx: idx_t, val: f32)
    -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_bind_double(
        prepared_statement: duckdb_prepared_statement,
        param_idx: idx_t,
        val: f64,
    ) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_bind_date(
        prepared_statement: duckdb_prepared_statement,
        param_idx: idx_t,
        val: duckdb_date,
    ) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_bind_time(
        prepared_statement: duckdb_prepared_statement,
        param_idx: idx_t,
        val: duckdb_time,
    ) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_bind_timestamp(
        prepared_statement: duckdb_prepared_statement,
        param_idx: idx_t,
        val: duckdb_timestamp,
    ) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_bind_timestamp_tz(
        prepared_statement: duckdb_prepared_statement,
        param_idx: idx_t,
        val: duckdb_timestamp,
    ) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_bind_interval(
        prepared_statement: duckdb_prepared_statement,
        param_idx: idx_t,
        val: duckdb_interval,
    ) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_bind_varchar(
        prepared_statement: duckdb_prepared_statement,
        param_idx: idx_t,
        val: *const ::std::os::raw::c_char,
    ) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_bind_varchar_length(
        prepared_statement: duckdb_prepared_statement,
        param_idx: idx_t,
        val: *const ::std::os::raw::c_char,
        length: idx_t,
    ) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_bind_blob(
        prepared_statement: duckdb_prepared_statement,
        param_idx: idx_t,
        data: *const ::std::os::raw::c_void,
        length: idx_t,
    ) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_bind_null(prepared_statement: duckdb_prepared_statement, param_idx: idx_t) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_execute_prepared(
        prepared_statement: duckdb_prepared_statement,
        out_result: *mut duckdb_result,
    ) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_execute_prepared_streaming(
        prepared_statement: duckdb_prepared_statement,
        out_result: *mut duckdb_result,
    ) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_extract_statements(
        connection: duckdb_connection,
        query: *const ::std::os::raw::c_char,
        out_extracted_statements: *mut duckdb_extracted_statements,
    ) -> idx_t;
}
unsafe extern "C" {
    pub fn duckdb_prepare_extracted_statement(
        connection: duckdb_connection,
        extracted_statements: duckdb_extracted_statements,
        index: idx_t,
        out_prepared_statement: *mut duckdb_prepared_statement,
    ) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_extract_statements_error(
        extracted_statements: duckdb_extracted_statements,
    ) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn duckdb_destroy_extracted(extracted_statements: *mut duckdb_extracted_statements);
}
unsafe extern "C" {
    pub fn duckdb_pending_prepared(
        prepared_statement: duckdb_prepared_statement,
        out_result: *mut duckdb_pending_result,
    ) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_pending_prepared_streaming(
        prepared_statement: duckdb_prepared_statement,
        out_result: *mut duckdb_pending_result,
    ) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_destroy_pending(pending_result: *mut duckdb_pending_result);
}
unsafe extern "C" {
    pub fn duckdb_pending_error(pending_result: duckdb_pending_result) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn duckdb_pending_execute_task(pending_result: duckdb_pending_result) -> duckdb_pending_state;
}
unsafe extern "C" {
    pub fn duckdb_pending_execute_check_state(pending_result: duckdb_pending_result) -> duckdb_pending_state;
}
unsafe extern "C" {
    pub fn duckdb_execute_pending(
        pending_result: duckdb_pending_result,
        out_result: *mut duckdb_result,
    ) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_pending_execution_is_finished(pending_state: duckdb_pending_state) -> bool;
}
unsafe extern "C" {
    pub fn duckdb_destroy_value(value: *mut duckdb_value);
}
unsafe extern "C" {
    pub fn duckdb_create_varchar(text: *const ::std::os::raw::c_char) -> duckdb_value;
}
unsafe extern "C" {
    pub fn duckdb_create_varchar_length(text: *const ::std::os::raw::c_char, length: idx_t) -> duckdb_value;
}
unsafe extern "C" {
    pub fn duckdb_create_bool(input: bool) -> duckdb_value;
}
unsafe extern "C" {
    pub fn duckdb_create_int8(input: i8) -> duckdb_value;
}
unsafe extern "C" {
    pub fn duckdb_create_uint8(input: u8) -> duckdb_value;
}
unsafe extern "C" {
    pub fn duckdb_create_int16(input: i16) -> duckdb_value;
}
unsafe extern "C" {
    pub fn duckdb_create_uint16(input: u16) -> duckdb_value;
}
unsafe extern "C" {
    pub fn duckdb_create_int32(input: i32) -> duckdb_value;
}
unsafe extern "C" {
    pub fn duckdb_create_uint32(input: u32) -> duckdb_value;
}
unsafe extern "C" {
    pub fn duckdb_create_uint64(input: u64) -> duckdb_value;
}
unsafe extern "C" {
    pub fn duckdb_create_int64(val: i64) -> duckdb_value;
}
unsafe extern "C" {
    pub fn duckdb_create_hugeint(input: duckdb_hugeint) -> duckdb_value;
}
unsafe extern "C" {
    pub fn duckdb_create_uhugeint(input: duckdb_uhugeint) -> duckdb_value;
}
unsafe extern "C" {
    pub fn duckdb_create_bignum(input: duckdb_bignum) -> duckdb_value;
}
unsafe extern "C" {
    pub fn duckdb_create_decimal(input: duckdb_decimal) -> duckdb_value;
}
unsafe extern "C" {
    pub fn duckdb_create_float(input: f32) -> duckdb_value;
}
unsafe extern "C" {
    pub fn duckdb_create_double(input: f64) -> duckdb_value;
}
unsafe extern "C" {
    pub fn duckdb_create_date(input: duckdb_date) -> duckdb_value;
}
unsafe extern "C" {
    pub fn duckdb_create_time(input: duckdb_time) -> duckdb_value;
}
unsafe extern "C" {
    pub fn duckdb_create_time_ns(input: duckdb_time_ns) -> duckdb_value;
}
unsafe extern "C" {
    pub fn duckdb_create_time_tz_value(value: duckdb_time_tz) -> duckdb_value;
}
unsafe extern "C" {
    pub fn duckdb_create_timestamp(input: duckdb_timestamp) -> duckdb_value;
}
unsafe extern "C" {
    pub fn duckdb_create_timestamp_tz(input: duckdb_timestamp) -> duckdb_value;
}
unsafe extern "C" {
    pub fn duckdb_create_timestamp_s(input: duckdb_timestamp_s) -> duckdb_value;
}
unsafe extern "C" {
    pub fn duckdb_create_timestamp_ms(input: duckdb_timestamp_ms) -> duckdb_value;
}
unsafe extern "C" {
    pub fn duckdb_create_timestamp_ns(input: duckdb_timestamp_ns) -> duckdb_value;
}
unsafe extern "C" {
    pub fn duckdb_create_interval(input: duckdb_interval) -> duckdb_value;
}
unsafe extern "C" {
    pub fn duckdb_create_blob(data: *const u8, length: idx_t) -> duckdb_value;
}
unsafe extern "C" {
    pub fn duckdb_create_bit(input: duckdb_bit) -> duckdb_value;
}
unsafe extern "C" {
    pub fn duckdb_create_uuid(input: duckdb_uhugeint) -> duckdb_value;
}
unsafe extern "C" {
    pub fn duckdb_get_bool(val: duckdb_value) -> bool;
}
unsafe extern "C" {
    pub fn duckdb_get_int8(val: duckdb_value) -> i8;
}
unsafe extern "C" {
    pub fn duckdb_get_uint8(val: duckdb_value) -> u8;
}
unsafe extern "C" {
    pub fn duckdb_get_int16(val: duckdb_value) -> i16;
}
unsafe extern "C" {
    pub fn duckdb_get_uint16(val: duckdb_value) -> u16;
}
unsafe extern "C" {
    pub fn duckdb_get_int32(val: duckdb_value) -> i32;
}
unsafe extern "C" {
    pub fn duckdb_get_uint32(val: duckdb_value) -> u32;
}
unsafe extern "C" {
    pub fn duckdb_get_int64(val: duckdb_value) -> i64;
}
unsafe extern "C" {
    pub fn duckdb_get_uint64(val: duckdb_value) -> u64;
}
unsafe extern "C" {
    pub fn duckdb_get_hugeint(val: duckdb_value) -> duckdb_hugeint;
}
unsafe extern "C" {
    pub fn duckdb_get_uhugeint(val: duckdb_value) -> duckdb_uhugeint;
}
unsafe extern "C" {
    pub fn duckdb_get_bignum(val: duckdb_value) -> duckdb_bignum;
}
unsafe extern "C" {
    pub fn duckdb_get_decimal(val: duckdb_value) -> duckdb_decimal;
}
unsafe extern "C" {
    pub fn duckdb_get_float(val: duckdb_value) -> f32;
}
unsafe extern "C" {
    pub fn duckdb_get_double(val: duckdb_value) -> f64;
}
unsafe extern "C" {
    pub fn duckdb_get_date(val: duckdb_value) -> duckdb_date;
}
unsafe extern "C" {
    pub fn duckdb_get_time(val: duckdb_value) -> duckdb_time;
}
unsafe extern "C" {
    pub fn duckdb_get_time_ns(val: duckdb_value) -> duckdb_time_ns;
}
unsafe extern "C" {
    pub fn duckdb_get_time_tz(val: duckdb_value) -> duckdb_time_tz;
}
unsafe extern "C" {
    pub fn duckdb_get_timestamp(val: duckdb_value) -> duckdb_timestamp;
}
unsafe extern "C" {
    pub fn duckdb_get_timestamp_tz(val: duckdb_value) -> duckdb_timestamp;
}
unsafe extern "C" {
    pub fn duckdb_get_timestamp_s(val: duckdb_value) -> duckdb_timestamp_s;
}
unsafe extern "C" {
    pub fn duckdb_get_timestamp_ms(val: duckdb_value) -> duckdb_timestamp_ms;
}
unsafe extern "C" {
    pub fn duckdb_get_timestamp_ns(val: duckdb_value) -> duckdb_timestamp_ns;
}
unsafe extern "C" {
    pub fn duckdb_get_interval(val: duckdb_value) -> duckdb_interval;
}
unsafe extern "C" {
    pub fn duckdb_get_value_type(val: duckdb_value) -> duckdb_logical_type;
}
unsafe extern "C" {
    pub fn duckdb_get_blob(val: duckdb_value) -> duckdb_blob;
}
unsafe extern "C" {
    pub fn duckdb_get_bit(val: duckdb_value) -> duckdb_bit;
}
unsafe extern "C" {
    pub fn duckdb_get_uuid(val: duckdb_value) -> duckdb_uhugeint;
}
unsafe extern "C" {
    pub fn duckdb_get_varchar(value: duckdb_value) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn duckdb_create_struct_value(type_: duckdb_logical_type, values: *mut duckdb_value) -> duckdb_value;
}
unsafe extern "C" {
    pub fn duckdb_create_list_value(
        type_: duckdb_logical_type,
        values: *mut duckdb_value,
        value_count: idx_t,
    ) -> duckdb_value;
}
unsafe extern "C" {
    pub fn duckdb_create_array_value(
        type_: duckdb_logical_type,
        values: *mut duckdb_value,
        value_count: idx_t,
    ) -> duckdb_value;
}
unsafe extern "C" {
    pub fn duckdb_create_map_value(
        map_type: duckdb_logical_type,
        keys: *mut duckdb_value,
        values: *mut duckdb_value,
        entry_count: idx_t,
    ) -> duckdb_value;
}
unsafe extern "C" {
    pub fn duckdb_create_union_value(
        union_type: duckdb_logical_type,
        tag_index: idx_t,
        value: duckdb_value,
    ) -> duckdb_value;
}
unsafe extern "C" {
    pub fn duckdb_get_map_size(value: duckdb_value) -> idx_t;
}
unsafe extern "C" {
    pub fn duckdb_get_map_key(value: duckdb_value, index: idx_t) -> duckdb_value;
}
unsafe extern "C" {
    pub fn duckdb_get_map_value(value: duckdb_value, index: idx_t) -> duckdb_value;
}
unsafe extern "C" {
    pub fn duckdb_is_null_value(value: duckdb_value) -> bool;
}
unsafe extern "C" {
    pub fn duckdb_create_null_value() -> duckdb_value;
}
unsafe extern "C" {
    pub fn duckdb_get_list_size(value: duckdb_value) -> idx_t;
}
unsafe extern "C" {
    pub fn duckdb_get_list_child(value: duckdb_value, index: idx_t) -> duckdb_value;
}
unsafe extern "C" {
    pub fn duckdb_create_enum_value(type_: duckdb_logical_type, value: u64) -> duckdb_value;
}
unsafe extern "C" {
    pub fn duckdb_get_enum_value(value: duckdb_value) -> u64;
}
unsafe extern "C" {
    pub fn duckdb_get_struct_child(value: duckdb_value, index: idx_t) -> duckdb_value;
}
unsafe extern "C" {
    pub fn duckdb_value_to_string(value: duckdb_value) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn duckdb_create_logical_type(type_: duckdb_type) -> duckdb_logical_type;
}
unsafe extern "C" {
    pub fn duckdb_logical_type_get_alias(type_: duckdb_logical_type) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn duckdb_logical_type_set_alias(type_: duckdb_logical_type, alias: *const ::std::os::raw::c_char);
}
unsafe extern "C" {
    pub fn duckdb_create_list_type(type_: duckdb_logical_type) -> duckdb_logical_type;
}
unsafe extern "C" {
    pub fn duckdb_create_array_type(type_: duckdb_logical_type, array_size: idx_t) -> duckdb_logical_type;
}
unsafe extern "C" {
    pub fn duckdb_create_map_type(
        key_type: duckdb_logical_type,
        value_type: duckdb_logical_type,
    ) -> duckdb_logical_type;
}
unsafe extern "C" {
    pub fn duckdb_create_union_type(
        member_types: *mut duckdb_logical_type,
        member_names: *mut *const ::std::os::raw::c_char,
        member_count: idx_t,
    ) -> duckdb_logical_type;
}
unsafe extern "C" {
    pub fn duckdb_create_struct_type(
        member_types: *mut duckdb_logical_type,
        member_names: *mut *const ::std::os::raw::c_char,
        member_count: idx_t,
    ) -> duckdb_logical_type;
}
unsafe extern "C" {
    pub fn duckdb_create_enum_type(
        member_names: *mut *const ::std::os::raw::c_char,
        member_count: idx_t,
    ) -> duckdb_logical_type;
}
unsafe extern "C" {
    pub fn duckdb_create_decimal_type(width: u8, scale: u8) -> duckdb_logical_type;
}
unsafe extern "C" {
    pub fn duckdb_get_type_id(type_: duckdb_logical_type) -> duckdb_type;
}
unsafe extern "C" {
    pub fn duckdb_decimal_width(type_: duckdb_logical_type) -> u8;
}
unsafe extern "C" {
    pub fn duckdb_decimal_scale(type_: duckdb_logical_type) -> u8;
}
unsafe extern "C" {
    pub fn duckdb_decimal_internal_type(type_: duckdb_logical_type) -> duckdb_type;
}
unsafe extern "C" {
    pub fn duckdb_enum_internal_type(type_: duckdb_logical_type) -> duckdb_type;
}
unsafe extern "C" {
    pub fn duckdb_enum_dictionary_size(type_: duckdb_logical_type) -> u32;
}
unsafe extern "C" {
    pub fn duckdb_enum_dictionary_value(type_: duckdb_logical_type, index: idx_t) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn duckdb_list_type_child_type(type_: duckdb_logical_type) -> duckdb_logical_type;
}
unsafe extern "C" {
    pub fn duckdb_array_type_child_type(type_: duckdb_logical_type) -> duckdb_logical_type;
}
unsafe extern "C" {
    pub fn duckdb_array_type_array_size(type_: duckdb_logical_type) -> idx_t;
}
unsafe extern "C" {
    pub fn duckdb_map_type_key_type(type_: duckdb_logical_type) -> duckdb_logical_type;
}
unsafe extern "C" {
    pub fn duckdb_map_type_value_type(type_: duckdb_logical_type) -> duckdb_logical_type;
}
unsafe extern "C" {
    pub fn duckdb_struct_type_child_count(type_: duckdb_logical_type) -> idx_t;
}
unsafe extern "C" {
    pub fn duckdb_struct_type_child_name(type_: duckdb_logical_type, index: idx_t) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn duckdb_struct_type_child_type(type_: duckdb_logical_type, index: idx_t) -> duckdb_logical_type;
}
unsafe extern "C" {
    pub fn duckdb_union_type_member_count(type_: duckdb_logical_type) -> idx_t;
}
unsafe extern "C" {
    pub fn duckdb_union_type_member_name(type_: duckdb_logical_type, index: idx_t) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn duckdb_union_type_member_type(type_: duckdb_logical_type, index: idx_t) -> duckdb_logical_type;
}
unsafe extern "C" {
    pub fn duckdb_destroy_logical_type(type_: *mut duckdb_logical_type);
}
unsafe extern "C" {
    pub fn duckdb_register_logical_type(
        con: duckdb_connection,
        type_: duckdb_logical_type,
        info: duckdb_create_type_info,
    ) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_create_data_chunk(types: *mut duckdb_logical_type, column_count: idx_t) -> duckdb_data_chunk;
}
unsafe extern "C" {
    pub fn duckdb_destroy_data_chunk(chunk: *mut duckdb_data_chunk);
}
unsafe extern "C" {
    pub fn duckdb_data_chunk_reset(chunk: duckdb_data_chunk);
}
unsafe extern "C" {
    pub fn duckdb_data_chunk_get_column_count(chunk: duckdb_data_chunk) -> idx_t;
}
unsafe extern "C" {
    pub fn duckdb_data_chunk_get_vector(chunk: duckdb_data_chunk, col_idx: idx_t) -> duckdb_vector;
}
unsafe extern "C" {
    pub fn duckdb_data_chunk_get_size(chunk: duckdb_data_chunk) -> idx_t;
}
unsafe extern "C" {
    pub fn duckdb_data_chunk_set_size(chunk: duckdb_data_chunk, size: idx_t);
}
unsafe extern "C" {
    pub fn duckdb_create_vector(type_: duckdb_logical_type, capacity: idx_t) -> duckdb_vector;
}
unsafe extern "C" {
    pub fn duckdb_destroy_vector(vector: *mut duckdb_vector);
}
unsafe extern "C" {
    pub fn duckdb_vector_get_column_type(vector: duckdb_vector) -> duckdb_logical_type;
}
unsafe extern "C" {
    pub fn duckdb_vector_get_data(vector: duckdb_vector) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn duckdb_vector_get_validity(vector: duckdb_vector) -> *mut u64;
}
unsafe extern "C" {
    pub fn duckdb_vector_ensure_validity_writable(vector: duckdb_vector);
}
unsafe extern "C" {
    pub fn duckdb_vector_assign_string_element(
        vector: duckdb_vector,
        index: idx_t,
        str_: *const ::std::os::raw::c_char,
    );
}
unsafe extern "C" {
    pub fn duckdb_vector_assign_string_element_len(
        vector: duckdb_vector,
        index: idx_t,
        str_: *const ::std::os::raw::c_char,
        str_len: idx_t,
    );
}
unsafe extern "C" {
    pub fn duckdb_unsafe_vector_assign_string_element_len(
        vector: duckdb_vector,
        index: idx_t,
        str_: *const ::std::os::raw::c_char,
        str_len: idx_t,
    );
}
unsafe extern "C" {
    pub fn duckdb_list_vector_get_child(vector: duckdb_vector) -> duckdb_vector;
}
unsafe extern "C" {
    pub fn duckdb_list_vector_get_size(vector: duckdb_vector) -> idx_t;
}
unsafe extern "C" {
    pub fn duckdb_list_vector_set_size(vector: duckdb_vector, size: idx_t) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_list_vector_reserve(vector: duckdb_vector, required_capacity: idx_t) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_struct_vector_get_child(vector: duckdb_vector, index: idx_t) -> duckdb_vector;
}
unsafe extern "C" {
    pub fn duckdb_array_vector_get_child(vector: duckdb_vector) -> duckdb_vector;
}
unsafe extern "C" {
    pub fn duckdb_slice_vector(vector: duckdb_vector, sel: duckdb_selection_vector, len: idx_t);
}
unsafe extern "C" {
    pub fn duckdb_vector_copy_sel(
        src: duckdb_vector,
        dst: duckdb_vector,
        sel: duckdb_selection_vector,
        src_count: idx_t,
        src_offset: idx_t,
        dst_offset: idx_t,
    );
}
unsafe extern "C" {
    pub fn duckdb_vector_reference_value(vector: duckdb_vector, value: duckdb_value);
}
unsafe extern "C" {
    pub fn duckdb_vector_reference_vector(to_vector: duckdb_vector, from_vector: duckdb_vector);
}
unsafe extern "C" {
    pub fn duckdb_validity_row_is_valid(validity: *mut u64, row: idx_t) -> bool;
}
unsafe extern "C" {
    pub fn duckdb_validity_set_row_validity(validity: *mut u64, row: idx_t, valid: bool);
}
unsafe extern "C" {
    pub fn duckdb_validity_set_row_invalid(validity: *mut u64, row: idx_t);
}
unsafe extern "C" {
    pub fn duckdb_validity_set_row_valid(validity: *mut u64, row: idx_t);
}
unsafe extern "C" {
    pub fn duckdb_create_scalar_function() -> duckdb_scalar_function;
}
unsafe extern "C" {
    pub fn duckdb_destroy_scalar_function(scalar_function: *mut duckdb_scalar_function);
}
unsafe extern "C" {
    pub fn duckdb_scalar_function_set_name(
        scalar_function: duckdb_scalar_function,
        name: *const ::std::os::raw::c_char,
    );
}
unsafe extern "C" {
    pub fn duckdb_scalar_function_set_varargs(scalar_function: duckdb_scalar_function, type_: duckdb_logical_type);
}
unsafe extern "C" {
    pub fn duckdb_scalar_function_set_special_handling(scalar_function: duckdb_scalar_function);
}
unsafe extern "C" {
    pub fn duckdb_scalar_function_set_volatile(scalar_function: duckdb_scalar_function);
}
unsafe extern "C" {
    pub fn duckdb_scalar_function_add_parameter(scalar_function: duckdb_scalar_function, type_: duckdb_logical_type);
}
unsafe extern "C" {
    pub fn duckdb_scalar_function_set_return_type(scalar_function: duckdb_scalar_function, type_: duckdb_logical_type);
}
unsafe extern "C" {
    pub fn duckdb_scalar_function_set_extra_info(
        scalar_function: duckdb_scalar_function,
        extra_info: *mut ::std::os::raw::c_void,
        destroy: duckdb_delete_callback_t,
    );
}
unsafe extern "C" {
    pub fn duckdb_scalar_function_set_bind(
        scalar_function: duckdb_scalar_function,
        bind: duckdb_scalar_function_bind_t,
    );
}
unsafe extern "C" {
    pub fn duckdb_scalar_function_set_bind_data(
        info: duckdb_bind_info,
        bind_data: *mut ::std::os::raw::c_void,
        destroy: duckdb_delete_callback_t,
    );
}
unsafe extern "C" {
    pub fn duckdb_scalar_function_set_bind_data_copy(info: duckdb_bind_info, copy: duckdb_copy_callback_t);
}
unsafe extern "C" {
    pub fn duckdb_scalar_function_bind_set_error(info: duckdb_bind_info, error: *const ::std::os::raw::c_char);
}
unsafe extern "C" {
    pub fn duckdb_scalar_function_set_function(
        scalar_function: duckdb_scalar_function,
        function: duckdb_scalar_function_t,
    );
}
unsafe extern "C" {
    pub fn duckdb_register_scalar_function(
        con: duckdb_connection,
        scalar_function: duckdb_scalar_function,
    ) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_scalar_function_get_extra_info(info: duckdb_function_info) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn duckdb_scalar_function_bind_get_extra_info(info: duckdb_bind_info) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn duckdb_scalar_function_get_bind_data(info: duckdb_function_info) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn duckdb_scalar_function_get_client_context(info: duckdb_bind_info, out_context: *mut duckdb_client_context);
}
unsafe extern "C" {
    pub fn duckdb_scalar_function_set_error(info: duckdb_function_info, error: *const ::std::os::raw::c_char);
}
unsafe extern "C" {
    pub fn duckdb_create_scalar_function_set(name: *const ::std::os::raw::c_char) -> duckdb_scalar_function_set;
}
unsafe extern "C" {
    pub fn duckdb_destroy_scalar_function_set(scalar_function_set: *mut duckdb_scalar_function_set);
}
unsafe extern "C" {
    pub fn duckdb_add_scalar_function_to_set(
        set: duckdb_scalar_function_set,
        function: duckdb_scalar_function,
    ) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_register_scalar_function_set(con: duckdb_connection, set: duckdb_scalar_function_set)
    -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_scalar_function_bind_get_argument_count(info: duckdb_bind_info) -> idx_t;
}
unsafe extern "C" {
    pub fn duckdb_scalar_function_bind_get_argument(info: duckdb_bind_info, index: idx_t) -> duckdb_expression;
}
unsafe extern "C" {
    pub fn duckdb_scalar_function_get_state(info: duckdb_function_info) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn duckdb_scalar_function_set_init(
        scalar_function: duckdb_scalar_function,
        init: duckdb_scalar_function_init_t,
    );
}
unsafe extern "C" {
    pub fn duckdb_scalar_function_init_set_error(info: duckdb_init_info, error: *const ::std::os::raw::c_char);
}
unsafe extern "C" {
    pub fn duckdb_scalar_function_init_set_state(
        info: duckdb_init_info,
        state: *mut ::std::os::raw::c_void,
        destroy: duckdb_delete_callback_t,
    );
}
unsafe extern "C" {
    pub fn duckdb_scalar_function_init_get_client_context(
        info: duckdb_init_info,
        out_context: *mut duckdb_client_context,
    );
}
unsafe extern "C" {
    pub fn duckdb_scalar_function_init_get_bind_data(info: duckdb_init_info) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn duckdb_scalar_function_init_get_extra_info(info: duckdb_init_info) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn duckdb_create_selection_vector(size: idx_t) -> duckdb_selection_vector;
}
unsafe extern "C" {
    pub fn duckdb_destroy_selection_vector(sel: duckdb_selection_vector);
}
unsafe extern "C" {
    pub fn duckdb_selection_vector_get_data_ptr(sel: duckdb_selection_vector) -> *mut sel_t;
}
unsafe extern "C" {
    pub fn duckdb_create_aggregate_function() -> duckdb_aggregate_function;
}
unsafe extern "C" {
    pub fn duckdb_destroy_aggregate_function(aggregate_function: *mut duckdb_aggregate_function);
}
unsafe extern "C" {
    pub fn duckdb_aggregate_function_set_name(
        aggregate_function: duckdb_aggregate_function,
        name: *const ::std::os::raw::c_char,
    );
}
unsafe extern "C" {
    pub fn duckdb_aggregate_function_add_parameter(
        aggregate_function: duckdb_aggregate_function,
        type_: duckdb_logical_type,
    );
}
unsafe extern "C" {
    pub fn duckdb_aggregate_function_set_return_type(
        aggregate_function: duckdb_aggregate_function,
        type_: duckdb_logical_type,
    );
}
unsafe extern "C" {
    pub fn duckdb_aggregate_function_set_functions(
        aggregate_function: duckdb_aggregate_function,
        state_size: duckdb_aggregate_state_size,
        state_init: duckdb_aggregate_init_t,
        update: duckdb_aggregate_update_t,
        combine: duckdb_aggregate_combine_t,
        finalize: duckdb_aggregate_finalize_t,
    );
}
unsafe extern "C" {
    pub fn duckdb_aggregate_function_set_destructor(
        aggregate_function: duckdb_aggregate_function,
        destroy: duckdb_aggregate_destroy_t,
    );
}
unsafe extern "C" {
    pub fn duckdb_register_aggregate_function(
        con: duckdb_connection,
        aggregate_function: duckdb_aggregate_function,
    ) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_aggregate_function_set_special_handling(aggregate_function: duckdb_aggregate_function);
}
unsafe extern "C" {
    pub fn duckdb_aggregate_function_set_extra_info(
        aggregate_function: duckdb_aggregate_function,
        extra_info: *mut ::std::os::raw::c_void,
        destroy: duckdb_delete_callback_t,
    );
}
unsafe extern "C" {
    pub fn duckdb_aggregate_function_get_extra_info(info: duckdb_function_info) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn duckdb_aggregate_function_set_error(info: duckdb_function_info, error: *const ::std::os::raw::c_char);
}
unsafe extern "C" {
    pub fn duckdb_create_aggregate_function_set(name: *const ::std::os::raw::c_char) -> duckdb_aggregate_function_set;
}
unsafe extern "C" {
    pub fn duckdb_destroy_aggregate_function_set(aggregate_function_set: *mut duckdb_aggregate_function_set);
}
unsafe extern "C" {
    pub fn duckdb_add_aggregate_function_to_set(
        set: duckdb_aggregate_function_set,
        function: duckdb_aggregate_function,
    ) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_register_aggregate_function_set(
        con: duckdb_connection,
        set: duckdb_aggregate_function_set,
    ) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_create_table_function() -> duckdb_table_function;
}
unsafe extern "C" {
    pub fn duckdb_destroy_table_function(table_function: *mut duckdb_table_function);
}
unsafe extern "C" {
    pub fn duckdb_table_function_set_name(table_function: duckdb_table_function, name: *const ::std::os::raw::c_char);
}
unsafe extern "C" {
    pub fn duckdb_table_function_add_parameter(table_function: duckdb_table_function, type_: duckdb_logical_type);
}
unsafe extern "C" {
    pub fn duckdb_table_function_add_named_parameter(
        table_function: duckdb_table_function,
        name: *const ::std::os::raw::c_char,
        type_: duckdb_logical_type,
    );
}
unsafe extern "C" {
    pub fn duckdb_table_function_set_extra_info(
        table_function: duckdb_table_function,
        extra_info: *mut ::std::os::raw::c_void,
        destroy: duckdb_delete_callback_t,
    );
}
unsafe extern "C" {
    pub fn duckdb_table_function_set_bind(table_function: duckdb_table_function, bind: duckdb_table_function_bind_t);
}
unsafe extern "C" {
    pub fn duckdb_table_function_set_init(table_function: duckdb_table_function, init: duckdb_table_function_init_t);
}
unsafe extern "C" {
    pub fn duckdb_table_function_set_local_init(
        table_function: duckdb_table_function,
        init: duckdb_table_function_init_t,
    );
}
unsafe extern "C" {
    pub fn duckdb_table_function_set_function(table_function: duckdb_table_function, function: duckdb_table_function_t);
}
unsafe extern "C" {
    pub fn duckdb_table_function_supports_projection_pushdown(table_function: duckdb_table_function, pushdown: bool);
}
unsafe extern "C" {
    pub fn duckdb_register_table_function(con: duckdb_connection, function: duckdb_table_function) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_bind_get_extra_info(info: duckdb_bind_info) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn duckdb_table_function_get_client_context(info: duckdb_bind_info, out_context: *mut duckdb_client_context);
}
unsafe extern "C" {
    pub fn duckdb_bind_add_result_column(
        info: duckdb_bind_info,
        name: *const ::std::os::raw::c_char,
        type_: duckdb_logical_type,
    );
}
unsafe extern "C" {
    pub fn duckdb_bind_get_parameter_count(info: duckdb_bind_info) -> idx_t;
}
unsafe extern "C" {
    pub fn duckdb_bind_get_parameter(info: duckdb_bind_info, index: idx_t) -> duckdb_value;
}
unsafe extern "C" {
    pub fn duckdb_bind_get_named_parameter(info: duckdb_bind_info, name: *const ::std::os::raw::c_char)
    -> duckdb_value;
}
unsafe extern "C" {
    pub fn duckdb_bind_set_bind_data(
        info: duckdb_bind_info,
        bind_data: *mut ::std::os::raw::c_void,
        destroy: duckdb_delete_callback_t,
    );
}
unsafe extern "C" {
    pub fn duckdb_bind_set_cardinality(info: duckdb_bind_info, cardinality: idx_t, is_exact: bool);
}
unsafe extern "C" {
    pub fn duckdb_bind_set_error(info: duckdb_bind_info, error: *const ::std::os::raw::c_char);
}
unsafe extern "C" {
    pub fn duckdb_init_get_extra_info(info: duckdb_init_info) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn duckdb_init_get_bind_data(info: duckdb_init_info) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn duckdb_init_set_init_data(
        info: duckdb_init_info,
        init_data: *mut ::std::os::raw::c_void,
        destroy: duckdb_delete_callback_t,
    );
}
unsafe extern "C" {
    pub fn duckdb_init_get_column_count(info: duckdb_init_info) -> idx_t;
}
unsafe extern "C" {
    pub fn duckdb_init_get_column_index(info: duckdb_init_info, column_index: idx_t) -> idx_t;
}
unsafe extern "C" {
    pub fn duckdb_init_set_max_threads(info: duckdb_init_info, max_threads: idx_t);
}
unsafe extern "C" {
    pub fn duckdb_init_set_error(info: duckdb_init_info, error: *const ::std::os::raw::c_char);
}
unsafe extern "C" {
    pub fn duckdb_function_get_extra_info(info: duckdb_function_info) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn duckdb_function_get_bind_data(info: duckdb_function_info) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn duckdb_function_get_init_data(info: duckdb_function_info) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn duckdb_function_get_local_init_data(info: duckdb_function_info) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn duckdb_function_set_error(info: duckdb_function_info, error: *const ::std::os::raw::c_char);
}
unsafe extern "C" {
    pub fn duckdb_add_replacement_scan(
        db: duckdb_database,
        replacement: duckdb_replacement_callback_t,
        extra_data: *mut ::std::os::raw::c_void,
        delete_callback: duckdb_delete_callback_t,
    );
}
unsafe extern "C" {
    pub fn duckdb_replacement_scan_set_function_name(
        info: duckdb_replacement_scan_info,
        function_name: *const ::std::os::raw::c_char,
    );
}
unsafe extern "C" {
    pub fn duckdb_replacement_scan_add_parameter(info: duckdb_replacement_scan_info, parameter: duckdb_value);
}
unsafe extern "C" {
    pub fn duckdb_replacement_scan_set_error(info: duckdb_replacement_scan_info, error: *const ::std::os::raw::c_char);
}
unsafe extern "C" {
    pub fn duckdb_get_profiling_info(connection: duckdb_connection) -> duckdb_profiling_info;
}
unsafe extern "C" {
    pub fn duckdb_profiling_info_get_value(
        info: duckdb_profiling_info,
        key: *const ::std::os::raw::c_char,
    ) -> duckdb_value;
}
unsafe extern "C" {
    pub fn duckdb_profiling_info_get_metrics(info: duckdb_profiling_info) -> duckdb_value;
}
unsafe extern "C" {
    pub fn duckdb_profiling_info_get_child_count(info: duckdb_profiling_info) -> idx_t;
}
unsafe extern "C" {
    pub fn duckdb_profiling_info_get_child(info: duckdb_profiling_info, index: idx_t) -> duckdb_profiling_info;
}
unsafe extern "C" {
    pub fn duckdb_appender_create(
        connection: duckdb_connection,
        schema: *const ::std::os::raw::c_char,
        table: *const ::std::os::raw::c_char,
        out_appender: *mut duckdb_appender,
    ) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_appender_create_ext(
        connection: duckdb_connection,
        catalog: *const ::std::os::raw::c_char,
        schema: *const ::std::os::raw::c_char,
        table: *const ::std::os::raw::c_char,
        out_appender: *mut duckdb_appender,
    ) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_appender_create_query(
        connection: duckdb_connection,
        query: *const ::std::os::raw::c_char,
        column_count: idx_t,
        types: *mut duckdb_logical_type,
        table_name: *const ::std::os::raw::c_char,
        column_names: *mut *const ::std::os::raw::c_char,
        out_appender: *mut duckdb_appender,
    ) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_appender_column_count(appender: duckdb_appender) -> idx_t;
}
unsafe extern "C" {
    pub fn duckdb_appender_column_type(appender: duckdb_appender, col_idx: idx_t) -> duckdb_logical_type;
}
unsafe extern "C" {
    pub fn duckdb_appender_error(appender: duckdb_appender) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn duckdb_appender_error_data(appender: duckdb_appender) -> duckdb_error_data;
}
unsafe extern "C" {
    pub fn duckdb_appender_flush(appender: duckdb_appender) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_appender_clear(appender: duckdb_appender) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_appender_close(appender: duckdb_appender) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_appender_destroy(appender: *mut duckdb_appender) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_appender_add_column(appender: duckdb_appender, name: *const ::std::os::raw::c_char) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_appender_clear_columns(appender: duckdb_appender) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_appender_begin_row(appender: duckdb_appender) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_appender_end_row(appender: duckdb_appender) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_append_default(appender: duckdb_appender) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_append_default_to_chunk(
        appender: duckdb_appender,
        chunk: duckdb_data_chunk,
        col: idx_t,
        row: idx_t,
    ) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_append_bool(appender: duckdb_appender, value: bool) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_append_int8(appender: duckdb_appender, value: i8) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_append_int16(appender: duckdb_appender, value: i16) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_append_int32(appender: duckdb_appender, value: i32) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_append_int64(appender: duckdb_appender, value: i64) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_append_hugeint(appender: duckdb_appender, value: duckdb_hugeint) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_append_uint8(appender: duckdb_appender, value: u8) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_append_uint16(appender: duckdb_appender, value: u16) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_append_uint32(appender: duckdb_appender, value: u32) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_append_uint64(appender: duckdb_appender, value: u64) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_append_uhugeint(appender: duckdb_appender, value: duckdb_uhugeint) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_append_float(appender: duckdb_appender, value: f32) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_append_double(appender: duckdb_appender, value: f64) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_append_date(appender: duckdb_appender, value: duckdb_date) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_append_time(appender: duckdb_appender, value: duckdb_time) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_append_timestamp(appender: duckdb_appender, value: duckdb_timestamp) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_append_interval(appender: duckdb_appender, value: duckdb_interval) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_append_varchar(appender: duckdb_appender, val: *const ::std::os::raw::c_char) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_append_varchar_length(
        appender: duckdb_appender,
        val: *const ::std::os::raw::c_char,
        length: idx_t,
    ) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_append_blob(
        appender: duckdb_appender,
        data: *const ::std::os::raw::c_void,
        length: idx_t,
    ) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_append_null(appender: duckdb_appender) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_append_value(appender: duckdb_appender, value: duckdb_value) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_append_data_chunk(appender: duckdb_appender, chunk: duckdb_data_chunk) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_table_description_create(
        connection: duckdb_connection,
        schema: *const ::std::os::raw::c_char,
        table: *const ::std::os::raw::c_char,
        out: *mut duckdb_table_description,
    ) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_table_description_create_ext(
        connection: duckdb_connection,
        catalog: *const ::std::os::raw::c_char,
        schema: *const ::std::os::raw::c_char,
        table: *const ::std::os::raw::c_char,
        out: *mut duckdb_table_description,
    ) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_table_description_destroy(table_description: *mut duckdb_table_description);
}
unsafe extern "C" {
    pub fn duckdb_table_description_error(table_description: duckdb_table_description)
    -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn duckdb_column_has_default(
        table_description: duckdb_table_description,
        index: idx_t,
        out: *mut bool,
    ) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_table_description_get_column_count(table_description: duckdb_table_description) -> idx_t;
}
unsafe extern "C" {
    pub fn duckdb_table_description_get_column_name(
        table_description: duckdb_table_description,
        index: idx_t,
    ) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn duckdb_table_description_get_column_type(
        table_description: duckdb_table_description,
        index: idx_t,
    ) -> duckdb_logical_type;
}
unsafe extern "C" {
    pub fn duckdb_to_arrow_schema(
        arrow_options: duckdb_arrow_options,
        types: *mut duckdb_logical_type,
        names: *mut *const ::std::os::raw::c_char,
        column_count: idx_t,
        out_schema: *mut ArrowSchema,
    ) -> duckdb_error_data;
}
unsafe extern "C" {
    pub fn duckdb_data_chunk_to_arrow(
        arrow_options: duckdb_arrow_options,
        chunk: duckdb_data_chunk,
        out_arrow_array: *mut ArrowArray,
    ) -> duckdb_error_data;
}
unsafe extern "C" {
    pub fn duckdb_schema_from_arrow(
        connection: duckdb_connection,
        schema: *mut ArrowSchema,
        out_types: *mut duckdb_arrow_converted_schema,
    ) -> duckdb_error_data;
}
unsafe extern "C" {
    pub fn duckdb_data_chunk_from_arrow(
        connection: duckdb_connection,
        arrow_array: *mut ArrowArray,
        converted_schema: duckdb_arrow_converted_schema,
        out_chunk: *mut duckdb_data_chunk,
    ) -> duckdb_error_data;
}
unsafe extern "C" {
    pub fn duckdb_destroy_arrow_converted_schema(arrow_converted_schema: *mut duckdb_arrow_converted_schema);
}
unsafe extern "C" {
    pub fn duckdb_query_arrow(
        connection: duckdb_connection,
        query: *const ::std::os::raw::c_char,
        out_result: *mut duckdb_arrow,
    ) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_query_arrow_schema(result: duckdb_arrow, out_schema: *mut duckdb_arrow_schema) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_prepared_arrow_schema(
        prepared: duckdb_prepared_statement,
        out_schema: *mut duckdb_arrow_schema,
    ) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_result_arrow_array(
        result: duckdb_result,
        chunk: duckdb_data_chunk,
        out_array: *mut duckdb_arrow_array,
    );
}
unsafe extern "C" {
    pub fn duckdb_query_arrow_array(result: duckdb_arrow, out_array: *mut duckdb_arrow_array) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_arrow_column_count(result: duckdb_arrow) -> idx_t;
}
unsafe extern "C" {
    pub fn duckdb_arrow_row_count(result: duckdb_arrow) -> idx_t;
}
unsafe extern "C" {
    pub fn duckdb_arrow_rows_changed(result: duckdb_arrow) -> idx_t;
}
unsafe extern "C" {
    pub fn duckdb_query_arrow_error(result: duckdb_arrow) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn duckdb_destroy_arrow(result: *mut duckdb_arrow);
}
unsafe extern "C" {
    pub fn duckdb_destroy_arrow_stream(stream_p: *mut duckdb_arrow_stream);
}
unsafe extern "C" {
    pub fn duckdb_execute_prepared_arrow(
        prepared_statement: duckdb_prepared_statement,
        out_result: *mut duckdb_arrow,
    ) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_arrow_scan(
        connection: duckdb_connection,
        table_name: *const ::std::os::raw::c_char,
        arrow: duckdb_arrow_stream,
    ) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_arrow_array_scan(
        connection: duckdb_connection,
        table_name: *const ::std::os::raw::c_char,
        arrow_schema: duckdb_arrow_schema,
        arrow_array: duckdb_arrow_array,
        out_stream: *mut duckdb_arrow_stream,
    ) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_execute_tasks(database: duckdb_database, max_tasks: idx_t);
}
unsafe extern "C" {
    pub fn duckdb_create_task_state(database: duckdb_database) -> duckdb_task_state;
}
unsafe extern "C" {
    pub fn duckdb_execute_tasks_state(state: duckdb_task_state);
}
unsafe extern "C" {
    pub fn duckdb_execute_n_tasks_state(state: duckdb_task_state, max_tasks: idx_t) -> idx_t;
}
unsafe extern "C" {
    pub fn duckdb_finish_execution(state: duckdb_task_state);
}
unsafe extern "C" {
    pub fn duckdb_task_state_is_finished(state: duckdb_task_state) -> bool;
}
unsafe extern "C" {
    pub fn duckdb_destroy_task_state(state: duckdb_task_state);
}
unsafe extern "C" {
    pub fn duckdb_execution_is_finished(con: duckdb_connection) -> bool;
}
unsafe extern "C" {
    pub fn duckdb_stream_fetch_chunk(result: duckdb_result) -> duckdb_data_chunk;
}
unsafe extern "C" {
    pub fn duckdb_fetch_chunk(result: duckdb_result) -> duckdb_data_chunk;
}
unsafe extern "C" {
    pub fn duckdb_create_cast_function() -> duckdb_cast_function;
}
unsafe extern "C" {
    pub fn duckdb_cast_function_set_source_type(cast_function: duckdb_cast_function, source_type: duckdb_logical_type);
}
unsafe extern "C" {
    pub fn duckdb_cast_function_set_target_type(cast_function: duckdb_cast_function, target_type: duckdb_logical_type);
}
unsafe extern "C" {
    pub fn duckdb_cast_function_set_implicit_cast_cost(cast_function: duckdb_cast_function, cost: i64);
}
unsafe extern "C" {
    pub fn duckdb_cast_function_set_function(cast_function: duckdb_cast_function, function: duckdb_cast_function_t);
}
unsafe extern "C" {
    pub fn duckdb_cast_function_set_extra_info(
        cast_function: duckdb_cast_function,
        extra_info: *mut ::std::os::raw::c_void,
        destroy: duckdb_delete_callback_t,
    );
}
unsafe extern "C" {
    pub fn duckdb_cast_function_get_extra_info(info: duckdb_function_info) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn duckdb_cast_function_get_cast_mode(info: duckdb_function_info) -> duckdb_cast_mode;
}
unsafe extern "C" {
    pub fn duckdb_cast_function_set_error(info: duckdb_function_info, error: *const ::std::os::raw::c_char);
}
unsafe extern "C" {
    pub fn duckdb_cast_function_set_row_error(
        info: duckdb_function_info,
        error: *const ::std::os::raw::c_char,
        row: idx_t,
        output: duckdb_vector,
    );
}
unsafe extern "C" {
    pub fn duckdb_register_cast_function(con: duckdb_connection, cast_function: duckdb_cast_function) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_destroy_cast_function(cast_function: *mut duckdb_cast_function);
}
unsafe extern "C" {
    pub fn duckdb_destroy_expression(expr: *mut duckdb_expression);
}
unsafe extern "C" {
    pub fn duckdb_expression_return_type(expr: duckdb_expression) -> duckdb_logical_type;
}
unsafe extern "C" {
    pub fn duckdb_expression_is_foldable(expr: duckdb_expression) -> bool;
}
unsafe extern "C" {
    pub fn duckdb_expression_fold(
        context: duckdb_client_context,
        expr: duckdb_expression,
        out_value: *mut duckdb_value,
    ) -> duckdb_error_data;
}
unsafe extern "C" {
    pub fn duckdb_client_context_get_file_system(context: duckdb_client_context) -> duckdb_file_system;
}
unsafe extern "C" {
    pub fn duckdb_destroy_file_system(file_system: *mut duckdb_file_system);
}
unsafe extern "C" {
    pub fn duckdb_file_system_error_data(file_system: duckdb_file_system) -> duckdb_error_data;
}
unsafe extern "C" {
    pub fn duckdb_file_system_open(
        file_system: duckdb_file_system,
        path: *const ::std::os::raw::c_char,
        options: duckdb_file_open_options,
        out_file: *mut duckdb_file_handle,
    ) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_create_file_open_options() -> duckdb_file_open_options;
}
unsafe extern "C" {
    pub fn duckdb_file_open_options_set_flag(
        options: duckdb_file_open_options,
        flag: duckdb_file_flag,
        value: bool,
    ) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_destroy_file_open_options(options: *mut duckdb_file_open_options);
}
unsafe extern "C" {
    pub fn duckdb_destroy_file_handle(file_handle: *mut duckdb_file_handle);
}
unsafe extern "C" {
    pub fn duckdb_file_handle_error_data(file_handle: duckdb_file_handle) -> duckdb_error_data;
}
unsafe extern "C" {
    pub fn duckdb_file_handle_read(
        file_handle: duckdb_file_handle,
        buffer: *mut ::std::os::raw::c_void,
        size: i64,
    ) -> i64;
}
unsafe extern "C" {
    pub fn duckdb_file_handle_write(
        file_handle: duckdb_file_handle,
        buffer: *const ::std::os::raw::c_void,
        size: i64,
    ) -> i64;
}
unsafe extern "C" {
    pub fn duckdb_file_handle_tell(file_handle: duckdb_file_handle) -> i64;
}
unsafe extern "C" {
    pub fn duckdb_file_handle_size(file_handle: duckdb_file_handle) -> i64;
}
unsafe extern "C" {
    pub fn duckdb_file_handle_seek(file_handle: duckdb_file_handle, position: i64) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_file_handle_sync(file_handle: duckdb_file_handle) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_file_handle_close(file_handle: duckdb_file_handle) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_create_config_option() -> duckdb_config_option;
}
unsafe extern "C" {
    pub fn duckdb_destroy_config_option(option: *mut duckdb_config_option);
}
unsafe extern "C" {
    pub fn duckdb_config_option_set_name(option: duckdb_config_option, name: *const ::std::os::raw::c_char);
}
unsafe extern "C" {
    pub fn duckdb_config_option_set_type(option: duckdb_config_option, type_: duckdb_logical_type);
}
unsafe extern "C" {
    pub fn duckdb_config_option_set_default_value(option: duckdb_config_option, default_value: duckdb_value);
}
unsafe extern "C" {
    pub fn duckdb_config_option_set_default_scope(
        option: duckdb_config_option,
        default_scope: duckdb_config_option_scope,
    );
}
unsafe extern "C" {
    pub fn duckdb_config_option_set_description(
        option: duckdb_config_option,
        description: *const ::std::os::raw::c_char,
    );
}
unsafe extern "C" {
    pub fn duckdb_register_config_option(connection: duckdb_connection, option: duckdb_config_option) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_client_context_get_config_option(
        context: duckdb_client_context,
        name: *const ::std::os::raw::c_char,
        out_scope: *mut duckdb_config_option_scope,
    ) -> duckdb_value;
}
unsafe extern "C" {
    pub fn duckdb_create_copy_function() -> duckdb_copy_function;
}
unsafe extern "C" {
    pub fn duckdb_copy_function_set_name(copy_function: duckdb_copy_function, name: *const ::std::os::raw::c_char);
}
unsafe extern "C" {
    pub fn duckdb_copy_function_set_extra_info(
        copy_function: duckdb_copy_function,
        extra_info: *mut ::std::os::raw::c_void,
        destructor: duckdb_delete_callback_t,
    );
}
unsafe extern "C" {
    pub fn duckdb_register_copy_function(
        connection: duckdb_connection,
        copy_function: duckdb_copy_function,
    ) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_destroy_copy_function(copy_function: *mut duckdb_copy_function);
}
unsafe extern "C" {
    pub fn duckdb_copy_function_set_bind(copy_function: duckdb_copy_function, bind: duckdb_copy_function_bind_t);
}
unsafe extern "C" {
    pub fn duckdb_copy_function_bind_set_error(
        info: duckdb_copy_function_bind_info,
        error: *const ::std::os::raw::c_char,
    );
}
unsafe extern "C" {
    pub fn duckdb_copy_function_bind_get_extra_info(
        info: duckdb_copy_function_bind_info,
    ) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn duckdb_copy_function_bind_get_client_context(info: duckdb_copy_function_bind_info) -> duckdb_client_context;
}
unsafe extern "C" {
    pub fn duckdb_copy_function_bind_get_column_count(info: duckdb_copy_function_bind_info) -> idx_t;
}
unsafe extern "C" {
    pub fn duckdb_copy_function_bind_get_column_type(
        info: duckdb_copy_function_bind_info,
        col_idx: idx_t,
    ) -> duckdb_logical_type;
}
unsafe extern "C" {
    pub fn duckdb_copy_function_bind_get_options(info: duckdb_copy_function_bind_info) -> duckdb_value;
}
unsafe extern "C" {
    pub fn duckdb_copy_function_bind_set_bind_data(
        info: duckdb_copy_function_bind_info,
        bind_data: *mut ::std::os::raw::c_void,
        destructor: duckdb_delete_callback_t,
    );
}
unsafe extern "C" {
    pub fn duckdb_copy_function_set_global_init(
        copy_function: duckdb_copy_function,
        init: duckdb_copy_function_global_init_t,
    );
}
unsafe extern "C" {
    pub fn duckdb_copy_function_global_init_set_error(
        info: duckdb_copy_function_global_init_info,
        error: *const ::std::os::raw::c_char,
    );
}
unsafe extern "C" {
    pub fn duckdb_copy_function_global_init_get_extra_info(
        info: duckdb_copy_function_global_init_info,
    ) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn duckdb_copy_function_global_init_get_client_context(
        info: duckdb_copy_function_global_init_info,
    ) -> duckdb_client_context;
}
unsafe extern "C" {
    pub fn duckdb_copy_function_global_init_get_bind_data(
        info: duckdb_copy_function_global_init_info,
    ) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn duckdb_copy_function_global_init_get_file_path(
        info: duckdb_copy_function_global_init_info,
    ) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn duckdb_copy_function_global_init_set_global_state(
        info: duckdb_copy_function_global_init_info,
        global_state: *mut ::std::os::raw::c_void,
        destructor: duckdb_delete_callback_t,
    );
}
unsafe extern "C" {
    pub fn duckdb_copy_function_set_sink(copy_function: duckdb_copy_function, function: duckdb_copy_function_sink_t);
}
unsafe extern "C" {
    pub fn duckdb_copy_function_sink_set_error(
        info: duckdb_copy_function_sink_info,
        error: *const ::std::os::raw::c_char,
    );
}
unsafe extern "C" {
    pub fn duckdb_copy_function_sink_get_extra_info(
        info: duckdb_copy_function_sink_info,
    ) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn duckdb_copy_function_sink_get_client_context(info: duckdb_copy_function_sink_info) -> duckdb_client_context;
}
unsafe extern "C" {
    pub fn duckdb_copy_function_sink_get_bind_data(info: duckdb_copy_function_sink_info)
    -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn duckdb_copy_function_sink_get_global_state(
        info: duckdb_copy_function_sink_info,
    ) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn duckdb_copy_function_set_finalize(
        copy_function: duckdb_copy_function,
        finalize: duckdb_copy_function_finalize_t,
    );
}
unsafe extern "C" {
    pub fn duckdb_copy_function_finalize_set_error(
        info: duckdb_copy_function_finalize_info,
        error: *const ::std::os::raw::c_char,
    );
}
unsafe extern "C" {
    pub fn duckdb_copy_function_finalize_get_extra_info(
        info: duckdb_copy_function_finalize_info,
    ) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn duckdb_copy_function_finalize_get_client_context(
        info: duckdb_copy_function_finalize_info,
    ) -> duckdb_client_context;
}
unsafe extern "C" {
    pub fn duckdb_copy_function_finalize_get_bind_data(
        info: duckdb_copy_function_finalize_info,
    ) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn duckdb_copy_function_finalize_get_global_state(
        info: duckdb_copy_function_finalize_info,
    ) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn duckdb_copy_function_set_copy_from_function(
        copy_function: duckdb_copy_function,
        table_function: duckdb_table_function,
    );
}
unsafe extern "C" {
    pub fn duckdb_table_function_bind_get_result_column_count(info: duckdb_bind_info) -> idx_t;
}
unsafe extern "C" {
    pub fn duckdb_table_function_bind_get_result_column_name(
        info: duckdb_bind_info,
        col_idx: idx_t,
    ) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn duckdb_table_function_bind_get_result_column_type(
        info: duckdb_bind_info,
        col_idx: idx_t,
    ) -> duckdb_logical_type;
}
unsafe extern "C" {
    pub fn duckdb_client_context_get_catalog(
        context: duckdb_client_context,
        catalog_name: *const ::std::os::raw::c_char,
    ) -> duckdb_catalog;
}
unsafe extern "C" {
    pub fn duckdb_catalog_get_type_name(catalog: duckdb_catalog) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn duckdb_catalog_get_entry(
        catalog: duckdb_catalog,
        context: duckdb_client_context,
        entry_type: duckdb_catalog_entry_type,
        schema_name: *const ::std::os::raw::c_char,
        entry_name: *const ::std::os::raw::c_char,
    ) -> duckdb_catalog_entry;
}
unsafe extern "C" {
    pub fn duckdb_destroy_catalog(catalog: *mut duckdb_catalog);
}
unsafe extern "C" {
    pub fn duckdb_catalog_entry_get_type(entry: duckdb_catalog_entry) -> duckdb_catalog_entry_type;
}
unsafe extern "C" {
    pub fn duckdb_catalog_entry_get_name(entry: duckdb_catalog_entry) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn duckdb_destroy_catalog_entry(entry: *mut duckdb_catalog_entry);
}
unsafe extern "C" {
    pub fn duckdb_create_log_storage() -> duckdb_log_storage;
}
unsafe extern "C" {
    pub fn duckdb_destroy_log_storage(log_storage: *mut duckdb_log_storage);
}
unsafe extern "C" {
    pub fn duckdb_log_storage_set_write_log_entry(
        log_storage: duckdb_log_storage,
        function: duckdb_logger_write_log_entry_t,
    );
}
unsafe extern "C" {
    pub fn duckdb_log_storage_set_extra_data(
        log_storage: duckdb_log_storage,
        extra_data: *mut ::std::os::raw::c_void,
        delete_callback: duckdb_delete_callback_t,
    );
}
unsafe extern "C" {
    pub fn duckdb_log_storage_set_name(log_storage: duckdb_log_storage, name: *const ::std::os::raw::c_char);
}
unsafe extern "C" {
    pub fn duckdb_register_log_storage(database: duckdb_database, log_storage: duckdb_log_storage) -> duckdb_state;
}
unsafe extern "C" {
    pub fn duckdb_geometry_type_get_crs(type_: duckdb_logical_type) -> *mut ::std::os::raw::c_char;
}
