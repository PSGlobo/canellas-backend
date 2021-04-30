use actix_web::HttpResponse;

/// Endpoint used by external services to check if this service is available.
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[cfg(test)]
mod tests {
    use super::health_check;

    #[actix_rt::test]
    async fn it_works() {
        assert!(health_check().await.status().is_success());
    }
}
