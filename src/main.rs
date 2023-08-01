use actix_web::{get, App, HttpServer, Responder, web, HttpResponse};
use std::sync::Mutex;
use products::models::{Product};

mod products;


use products::imports::read_products;

struct AppState {
    products: Mutex<Vec<Product>>,
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json(data.products.lock().unwrap().to_vec())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_data = actix_web::web::Data::new(AppState {
        products: Mutex::new(read_products(String::from("products.csv")))
    });


    HttpServer::new(move || {
        App::new().
            app_data(app_data.clone()).
            service(index)
    }).
        bind(("127.0.0.1", 8086))?.
        run().
        await
}
