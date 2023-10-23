use actix_web::{HttpResponse, post, web};
use serde::Deserialize;
use crate::controllers::ldap::ldap_login;
use crate::controllers::jwt::encode_jwt;
use crate::db::operations_users::{get_student_by_studnet_number, insert_student};
use crate::models::user::{NewUser, LdapUser};
use std::env;

#[derive(Deserialize)]
pub struct AuthPost {
    pub username: Option<String>,
    pub password: Option<String>,
}

#[post("/login")]
pub async fn login(body: web::Json<AuthPost>) -> HttpResponse {
    let credentials: AuthPost = body.into_inner();
    
    let username = match credentials.username {
        Some(n) => n,
        None => return HttpResponse::Unauthorized().finish()
    };

    let password = match credentials.password {
        Some(n) => n,
        None => return HttpResponse::Unauthorized().finish()
    };

    // admin special case
    if username.eq("admin") {
        let admin_pw = env::var("ADMIN_PASSWORD").expect("ADMIN_PASSWORD must be set");
        if password == admin_pw {
            return match encode_jwt("admin".to_string()) {
                Ok(token) => HttpResponse::Ok().body(token),
                Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
            };
        }
    }

    let ldap_dn_option = match ldap_login(username.clone(), password).await {
        Ok(b) => b,
        Err(e) => { println!("ERROR: {:#?}", e); None },
    };

    let ldap_dn = match ldap_dn_option {
        Some(ldap_dn) => ldap_dn,
        None => return HttpResponse::Unauthorized().finish()
    };
    
    // insert new student
    if let None = get_student_by_studnet_number(username.clone()).ok() {
        let new_student = NewUser::from(LdapUser {
            username: username.clone(), 
            ldap_dn 
        });
        if let Err(e) = insert_student(new_student) {
            return HttpResponse::InternalServerError().json(e.to_string());
        }
    };


    match encode_jwt(username) {
        Ok(token) => HttpResponse::Ok().body(token),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}