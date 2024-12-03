use serde::Serialize;

/// ### Generic response data model for project.
/// `data` :  your data vec.
///
/// ### Example
///
/// ```
/// use nextera_utils::models::response_data::ResponseData;
///
/// let res_data = ResponseData::<i32>{ data: vec![1,2,3], total: 3};
/// assert_eq!(res_data.data.len(), 3);
/// assert_eq!(res_data.total, 3);
/// ```
#[derive(Serialize)]
pub struct ResponseData<T> {
    pub data: Vec<T>,
    pub total: i64,
}
