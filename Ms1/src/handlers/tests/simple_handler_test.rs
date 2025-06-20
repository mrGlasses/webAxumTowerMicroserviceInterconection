use crate::domain::general::{FilterParams, Message, Params};
use crate::handlers::simple_handler::*;
use axum::body::HttpBody;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{extract::Query, http::HeaderMap, Json};
use httpmock::prelude::*;

#[tokio::test]
async fn test_get_pong() {
    let response = get_pong().await;
    let body = response.into_body().collect().await.unwrap().to_bytes();

    assert_eq!(&body[..], b"PONG!");
}

#[tokio::test]
async fn test_call_external_service_ok() {
    let server = MockServer::start_async().await;

    let hello_mock = server
        .mock_async(|when, then| {
            when.method("GET").path("/pong");
            then.status(200)
                .header("content-type", "text/html; charset=UTF-8")
                .body(r#"{"code": 200, "message_text": "PONG"}"#);
        })
        .await;

    unsafe {
        std::env::set_var("EXTERNAL_SERVICE_URL", server.url(""));
    }

    let response = call_external_service().await;

    hello_mock.assert();

    assert_eq!(response.into_response().status(), StatusCode::OK);
}

#[tokio::test]
async fn test_call_external_service_fail() {
    unsafe {
        std::env::set_var("EXTERNAL_SERVICE_URL", "localhost:99999");
    }

    let response = call_external_service().await;

    assert_ne!(response.into_response().status(), StatusCode::OK);
}

#[tokio::test]
async fn test_protected_route_no_auth() {
    let headers = HeaderMap::new();
    let response = protected_route(headers).await;

    assert_eq!(response.into_response().status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_protected_route_with_auth() {
    let mut headers = HeaderMap::new();
    headers.insert(
        "X-Custom-Header",
        axum::http::HeaderValue::from_str("secret-value").unwrap(),
    );

    let response = protected_route(headers).await;
    assert_eq!(response.into_response().status(), StatusCode::OK);
}

#[tokio::test]
async fn test_get_params() {
    let param_in = Path(Params {
        param_1: 1,
        param_2: "test2".to_string(),
    });
    let response = get_params(param_in).await;
    let body = response.into_body().collect().await.unwrap().to_bytes();

    assert_eq!(body, "Parameter 1: 1, Parameter 2: test2");
}

#[tokio::test]
async fn test_get_question() {
    let query = Query(FilterParams {
        name: Option::from("Jack".to_string()),
        age: Option::from(25),
        active: Option::from(true),
    });

    let response = get_question(query).await;
    let body = response.into_body().collect().await.unwrap().to_bytes();
    match std::str::from_utf8(&body[..]) {
        Ok(s) => {
            assert!(s.contains("Jack"));
            assert!(s.contains("25"));
            assert!(s.contains("true"));
        }
        Err(e) => panic!("Error: {}", e),
    }
}

#[tokio::test]
async fn test_post_body_data() {
    let body_data = Message {
        code: 777,
        message_text: "Received body data successfully!".to_string(),
    };

    let response = post_body_data(Json(body_data)).await;
    let body = response.into_body().collect().await.unwrap().to_bytes();

    assert_eq!(
        body,
        "Received message with code: 777, text: Received body data successfully!".as_bytes()
    );
}
