use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde_json::json;
use crate::clients::Client;

#[get("/clients")]
async fn find_all() -> impl Responder {
    HttpResponse::Ok().json(vec![
        Client {
            id: 1,
            first_name: "Vent".to_string(),
            last_name: "Grey".to_string(),
            email: "VentGrey@ventmail.com".to_string(),
            paid: 2500,
            age: 21
        },

        Client {
            id: 2,
            first_name: "Vent".to_string(),
            last_name: "Coloured".to_string(),
            email: "VentColoured@ventmail.com".to_string(),
            paid: 2500,
            age: 22
        },
    ])
}

#[get("/clients/{id}")]
async fn find() -> impl Responder {
    HttpResponse::Ok().json(Client {
        id: 2,
        first_name: "Vent".to_string(),
        last_name: "Coloured".to_string(),
        email: "VentColoured@ventmail.com".to_string(),
        paid: 2500,
        age: 22
    })
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(find_all);
    config.service(find);
}
