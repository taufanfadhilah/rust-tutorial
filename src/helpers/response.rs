use serde::Serialize;
use serde_json::json;

pub fn create_response<T: Serialize>(success: bool, message: &str, data: T) -> String {
    json!({
        "success": success,
        "message": message,
        "data": data
    })
    .to_string()
}
