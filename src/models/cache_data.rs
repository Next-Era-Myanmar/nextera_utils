use serde::{Deserialize, Serialize};

/// ### Generic cache data model for project.
/// `data` :  your data vec.
///
/// ### Example
///
/// ```
/// use nextera_utils::models::cache_data::CacheData;
///
/// let res_data = CacheData::<i32>{ data: vec![1,2,3], total: 3};
/// assert_eq!(res_data.data.len(), 3);
/// assert_eq!(res_data.total, 3);
/// ```
#[derive(Serialize, Deserialize)]
pub struct CacheData<T> {
    pub data: Vec<T>,
    pub total: i64,
}
