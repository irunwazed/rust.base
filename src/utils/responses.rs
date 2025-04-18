use axum::{
    body::Body,
    http::{Response, StatusCode},
};
use serde::Serialize;
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

pub fn response_get_success<T>(data: T) -> Response<Body>
where
    T: Serialize,
{
    let body = json!({
        "status": true,
        "message": "Berhasil mendapatkan data",
        "data": data
    });

    return Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(Body::from(serde_json::to_string(&body).unwrap()))
        .unwrap()
}

pub fn response_get_fail<T>(data: T) -> Response<Body>
where
    T: Serialize,
{
    let body = json!({
        "status": true,
        "message": "Error load data",
        "data": data
    });

    return Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .header("Content-Type", "application/json")
        .body(Body::from(serde_json::to_string(&body).unwrap()))
        .unwrap()
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
