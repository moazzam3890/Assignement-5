use actix_web::{web,App,HttpServer, Result};
use serde::Deserialize;

#[derive(Deserialize)]
struct Num {
    number: u32,
}

async fn gettin(num: web::Path<Num>) -> Result<String> {
    let result = num.number + 50;
    Ok(format!("You've entered {} but answer after adding 50 is: {}", num.number, result))
}

#[actix_rt::main]
async fn main() ->std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/get/{number}", web::get().to(gettin))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}