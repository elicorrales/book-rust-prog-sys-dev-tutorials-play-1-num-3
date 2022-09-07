use actix_web::{get, post, web, App, HttpServer, Responder, HttpResponse};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("\n\n\nStarting Web Server....\n\n\n");

    let my_server_closure = || {
        App::new()
            .service(anyhello)
            .service(hello)
            .service(index)
            .route("/", web::get().to(my_root_route))
    };

    let server = 
        HttpServer::new(my_server_closure);

    server.bind(("127.0.0.1",8080))?
                .run()
                .await

}

#[get("/hello")]
async fn anyhello() -> impl Responder {
    
    HttpResponse::Ok().body(
        "<h1>Hello to anyone</h1> "
    )
    
}

#[get("/hello/{name}")]
async fn hello(name: web::Path<String>) -> impl Responder {
    
    HttpResponse::Ok().body(
        "<h1>Hello ".to_string() + &name.to_string() + "</h1> "
    )
    
}

#[get("/index")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body(
        r#"
            <title>This is The Index Page</title>
            <h1>This is The Index Page</h1>
        "#
    )

}

async fn my_root_route() -> impl Responder {
    HttpResponse::Ok().body(
        r#"
            <title>This is The Title</title>
            <form>
            <input type="text"/>
            <input type="text"/>
            <button type="submit"/>
            </form>
        "#
    )
}