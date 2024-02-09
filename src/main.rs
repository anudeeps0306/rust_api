use actix_web::{get, patch, post, App, HttpResponse, HttpServer, Responder};

#[get("/product")]
async fn get_product() -> impl Responder {
    HttpResponse::Ok().body("Pizza available")
}

#[post("/buyproduct")]
async fn buyproduct() -> impl Responder {
    HttpResponse::Ok().body("working")
}

#[post("/updateproduct/{id}")]
async fn updateproduct() -> impl Responder {
    HttpResponse::Ok().body("Working")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(get_product)
        .service(buyproduct)
        .service(updateproduct)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
