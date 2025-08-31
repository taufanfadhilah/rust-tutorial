use axum::extract::Query;
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct Pagination {
    pub page: Option<usize>,
    pub per_page: Option<usize>,
}

pub fn create_response<T: Serialize>(success: bool, message: &str, data: T) -> String {
    json!({
        "success": success,
        "message": message,
        "data": data
    })
    .to_string()
}

pub fn create_pagination_metadata<T: Serialize>(
    data: T,
    total: usize,
    query: Query<Pagination>,
) -> serde_json::Value {
    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(10);
    let last_page = (total as f64 / per_page as f64).ceil() as usize;
    let from = if total == 0 {
        0
    } else {
        (page - 1) * per_page + 1
    };
    let to = usize::min(page * per_page, total);

    json!({
        "data": data,
        "total": total,
        "per_page": per_page,
        "current_page": page,
        "last_page": last_page,
        "from": from,
        "to": to
    })
}
