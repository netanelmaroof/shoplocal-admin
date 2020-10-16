use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
mod logger;
mod database;

//SERVICE DESCRIPTION
//The admin backend service, which implements CRUD functionality. 

//DB APP DATA SETUP
struct AppState {
    database_manager: DatabaseManager
}

struct DatabaseManager {
    database: mongodb::Database
}

impl DatabaseManager {
    pub fn new(db: mongodb::Database) -> Self {
        DatabaseManager {
            database: db
        }
    }
}

//End of setup


async fn hello() -> impl Responder {
    let mut l = logger::DefaultLogger::new();

    l.log_info("Hello hit");

    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();

    let db = database::start();

    HttpServer::new(move || {
        App::new()
            .service(
                web::scope("/admin")

                .route("/hello", web::get().to(hello))
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}