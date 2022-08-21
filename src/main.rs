#[macro_use] extern crate rocket;

use rocket::serde::json::{Json};

use reqwest;

extern crate dotenv;
use dotenv::dotenv;

use exante_api::jwt_token::{AssetInfo, ext_jwt};
//   jwt_token::{AssetInfo, ext_jwt};

#[get("/<asset>")]
    async fn world(asset: &str) -> Json<AssetInfo> {
        let jwt = ext_jwt();
        println!("jwt: {}", jwt);
        let url = format!("{}{}?token={}", "https://api-demo.exante.eu/md/3.0/symbols/", asset, jwt);
        let response = reqwest::get(url).await;
        let response_body = response.unwrap();
        let json = response_body.json::<AssetInfo>().await;
        let json_body = json.unwrap();
        Json(json_body)
    }

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    dotenv().ok();
    let _rocket = rocket::build()
        .mount("/hello", routes![world])
        .launch()
        .await?;

    Ok(())
}