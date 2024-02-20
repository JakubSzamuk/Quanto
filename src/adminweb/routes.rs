use actix_web::{get, HttpResponse, post, Responder};

#[get("/admin/dev")]
async fn get_api_routes() -> impl Responder {



    // HttpResponse::Ok().body("Hello world!")
}
#[post("/dev")]
async fn add_api_route() -> impl Responder {



    // HttpResponse::Ok().body("Hello world!")
}