use log::{info};
use std::env;
use actix_web::{web, Error, HttpResponse, guard, web::ReqData};
use crate::{AuthInfo};
use std::rc::Rc;

/// This function initiates logging
pub fn initialise_logging() {
    env::set_var("TOKENSERVICE_URL", "http://localhost:5000");
    let _result = env_logger::try_init();
    dotenv::dotenv().ok();
}

pub async fn check_health(auth_info: Option<ReqData<Rc<AuthInfo>>>) -> Result<HttpResponse, Error> {
    let auth_info = auth_info.unwrap().get_data();
    info!("UserID: {0}, AppID: {1}", auth_info.0, auth_info.1);
    Ok(HttpResponse::Ok().json("Service is reachable"))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/storeservice/health/")
            .name("health_check")
            .guard(guard::Header("content-type", "application/json"))
            .route(web::get().to(check_health))
    );
}

// This is intended to return the login cookie
pub fn get_login_coockie() -> String {
    let coockie_string: String = "bearer=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiIsImtpZCI6IjIifQ.eyJ1c2VyX2lkIjoyMCwiYXBwX2lkIjoyLCJhdXRob3JpemF0aW9uX2lkIjoxOSwidG9rZW5faWQiOjIwMjUsInBlcm1pc3Npb25fY29kZXMiOltdLCJleHAiOjE2ODQyMTgzNDF9.4tgyEYVqUZA1wSQ3iPjCozesB1gqi41VxtB8dcsF__i6oWtvncwOAlYEJfTHC0wuElAPc956u63rCCyIPt6UCA; Path=/;".to_string();
    coockie_string
}
