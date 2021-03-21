use std::collections::BTreeMap;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use handlebars::Handlebars;
use std::iter::FromIterator;

#[get("/")]
async fn hello(hb: web::Data<Handlebars<'_>>) -> impl Responder {
    //let mut data = BTreeMap::new();
    let iter = vec![("title", "Fun Stuff Now!"),
                    ("parent", "base")];
    let data = BTreeMap::from_iter(iter);

    let body = hb.render("index", &data).unwrap();

    HttpResponse::Ok().body(body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut handlebars = Handlebars::new();
    handlebars.set_dev_mode(true);

    // register template using given name
    handlebars.register_template_file("index", "./html/index.hbs").unwrap();
    handlebars.register_template_file("base", "./html/base.hbs").unwrap();


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
