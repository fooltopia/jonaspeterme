use std::collections::BTreeMap;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use handlebars::Handlebars;

#[get("/")]
async fn hello(hb: web::Data<Handlebars<'_>>) -> impl Responder {
    let mut data = BTreeMap::new();
    data.insert("title", "Fun Stuff Today!");
    data.insert("parent", "base");

    let body = hb.render("index", &data).unwrap();

    HttpResponse::Ok().body(body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut handlebars = Handlebars::new();
    // render without register

    // register template using given name
    handlebars.register_template_file("index", "./html/index.hbs").ok();
    handlebars.register_template_file("base", "./html/base.hbs").ok();

    let hb_ref = web::Data::new(handlebars);


    HttpServer::new(move || {
        App::new()
            .app_data(hb_ref.clone())
            .service(hello)
    })
        .bind("127.0.0.1:3000")?
        .run()
        .await
}
