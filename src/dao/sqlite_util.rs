use sqlx::prelude::FromRow;

#[cfg(feature = "ssr")]
#[derive(Clone, FromRow, Debug)]
pub struct TestStruct {
    name: String,
    val: i32,
}
