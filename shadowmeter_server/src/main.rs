/*
 *  Copyright 2024 Fidelis Farm & Technologies, LLC.
 *  All Rights Reserved.
 *  See license information in LICENSE.
 */

#[macro_use] extern crate rocket;

use std::io;
use std::time::{SystemTime, UNIX_EPOCH};
use rocket::data::{Data, ToByteUnit};
use rocket::http::uri::Absolute;
use rocket::response::content::RawText;
use rocket::tokio::fs::{self, File};

#[post("/", data = "<data>")]
async fn upload(data: Data<'_>) -> io::Result<String> {


    let upload_filename = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(e) => format!("./spool/sm.{}.flow", e.as_nanos().to_string()),
        Err(_) => panic!("error: SystemTime::now()"),
    };
    data.open(1.gigabytes()).into_file(upload_filename).await?;
    Ok("success".to_string())
}

#[get("/")]
fn index() -> &'static str {
    "shadowmeter"
}

#[launch]
fn rocket() -> _ {

    rocket::build().mount("/", routes![index, upload])
}
