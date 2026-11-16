/*
Our statement: When we receive a GET request for /health_check we return a 200 OK response with no body
 */

#[tokio::test]
async fn health_check_works() {
    // Starts our application in the background
    spawn_app().await.expect("Failed to spawn our app.");

    // Reqwest can perform HTTP request against our application.
    let client = reqwest::Client::new();

    // Sending the actual request
    let response = client
        .get("http://localhost:3000/health_check")
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert if
    // the status is not 200 OK
    assert!(response.status().is_success());
    // or if we got a body!
    assert_eq!(Some(0), response.content_length());
}

// Launch application in the background
async fn spawn_app() -> std::io::Result<()> {
    todo!("Investigate axum TestServer to support reqwest testing!")
}