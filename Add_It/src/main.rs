use actix_web::{HttpRequest,Result,web,App,HttpServer};


async fn index(req: HttpRequest) -> String {
    let mut sum: i32 = req.match_info().query("param").parse().unwrap();
    sum=sum+10;
    format!("Output= {}",sum)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {;

    HttpServer::new(|| {
        App::new().route(
            "/users/{param}",
            web::get().to(index),
        )
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}