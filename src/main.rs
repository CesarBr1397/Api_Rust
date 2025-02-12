use actix_web::{web, get, http::header, HttpResponse, HttpServer, App, Responder};


#[get("/usuario")]
async fn usuario() -> HttpResponse{
    HttpResponse::Ok()
    .content_type(header::ContentType::json())
    .body(
        r#"
        {
        "id": 1,
        "nombre": "Cesar",
        "apellido": "Vazquez"
        }
        "#,
    )
}

async fn saludo() -> HttpResponse{
    HttpResponse::Ok().content_type(header::ContentType::html()).body("Hola mundo")
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::get().to(saludo))
    .service(usuario))
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}