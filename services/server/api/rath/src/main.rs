




// https://www.rust-lang.org/learn
// https://dzone.com/articles/creating-a-rest-api-in-rust-using-rocket-and-diese
// https://auth0.com/blog/build-an-api-in-rust-with-jwt-authentication-using-actix-web/
// https://fdeantoni.medium.com/rust-actix-diesel-sqlite-d67a1c3ef0e
// serve pre-trained core AI models with rust using tch, actix, rocket, hyper, gotham, docker and k8s



use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
	println!("starting server");
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind("127.0.0.1:8587")?
    .run()
    .await
}
