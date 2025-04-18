use axum::{
    body::Body,
    http::{Response, StatusCode},
};
use serde_json::json;

// pub fn response_error(message: impl Into<String>) -> Response<Body> {
//     let body = json!({
//         "status": false,
//         "message": message.into()
//     });
//     return Response::builder()
//         .status(StatusCode::INTERNAL_SERVER_ERROR)
//         .header("Content-Type", "application/json")
//         .body(Body::from(body.to_string()))
//         .unwrap();
// }

pub fn response_invalid(message: impl Into<String>) -> Response<Body> {
    let body = json!({
        "status": false,
        "message": message.into()
    });
    return Response::builder()
        .status(StatusCode::NOT_ACCEPTABLE)
        .header("Content-Type", "application/json")
        .body(Body::from(body.to_string()))
        .unwrap();
}

pub fn response_bad_request(message: impl Into<String>) -> Response<Body> {
    let body = json!({
        "status": false,
        "message": message.into()
    });
    return Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .header("Content-Type", "application/json")
        .body(Body::from(body.to_string()))
        .unwrap();
}

pub fn response_get_success(body: impl Into<String>) -> Response<Body> {
    return Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(Body::from(body.into()))
        .unwrap();
}

// pub fn response_post_success(body: impl Into<String>) -> Response<Body> {
//   return Response::builder()
//       .status(StatusCode::CREATED)
//       .header("Content-Type", "application/json")
//       .body(Body::from(body.into()))
//       .unwrap()
// }

// pub fn response_put_success(body: impl Into<String>) -> Response<Body> {
//   return Response::builder()
//       .status(StatusCode::OK)
//       .header("Content-Type", "application/json")
//       .body(Body::from(body.into()))
//       .unwrap()
// }

// pub fn response_delete_success(body: impl Into<String>) -> Response<Body> {
//   return Response::builder()
//       .status(StatusCode::OK)
//       .header("Content-Type", "application/json")
//       .body(Body::from(body.into()))
//       .unwrap()
// }
