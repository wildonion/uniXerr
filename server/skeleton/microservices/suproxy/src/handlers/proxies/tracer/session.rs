


use uuid::Uuid;
use actix_web::{web, get, post, Error, HttpRequest, HttpResponse};
use actix::prelude::*;
use crate::constants;
use crate::utils::response::{ResponseBody, ServiceError};
use crate::handlers::{
    db::cass::establish as cass,
    db::cass::schemas::device::GPS,
};





#[post("/uniXerr/api/tracer/add")]
async fn add_data(req: HttpRequest, cass_sess: web::Data<cass::CassSession>, device: web::Json<GPS>) -> Result<HttpResponse, Error>{
    
    let cass_session = cass_sess.into_inner();
    let ip = req.peer_addr().unwrap().ip();
    let port = req.peer_addr().unwrap().port();
    Ok(HttpResponse::Ok()
            .json(ResponseBody::new(
                constants::MESSAGE_REGISTER_SUCCESS,
                GPS::default(),
            ))
            .into_body(),
    )

}



pub fn tracer_balancer_init(config: &mut web::ServiceConfig){
    config.service(add_data);
}
