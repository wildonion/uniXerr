



use actix_web::{Error, HttpRequest, HttpResponse, Result, get, post, web};
use crate::utils::response::{ResponseBody, ServiceError};
use crate::schemas::device::GPS;
use crate::constants;





#[post("/uniXerr/api/tracer/device/register")] //-- required fields : username + phone_number + sex + age + email
async fn register(req: HttpRequest, device: web::Json<GPS>) -> Result<HttpResponse, Error>{

    Ok(HttpResponse::Ok()
            .json(ResponseBody::new(
                constants::MESSAGE_REGISTER_SUCCESS,
                GPS::default(),
            ))
            .into_body(),
    )

}



pub fn init_routes(config: &mut web::ServiceConfig){
    config.service(register);
}
