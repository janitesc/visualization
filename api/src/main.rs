extern crate serde_json;
use serde_json::{Value as JsonValue, json, from_str};
use std::{cmp, fs::File};
use serde_json::{Number, Value};
use serde_json::Result;
use serde::{Deserialize, Serialize};
use actix_cors::Cors;
use actix_web::web::Json;
use actix_web::{middleware::Logger, App, HttpResponse, HttpServer, Responder};
use env_logger::Env;
use rand::Rng;
use std::io::Read;
use std::path::Path;
use std::fs;
use std::env;

/// Generates a random set of genes
///
/// It starts with a random length of a genomic region. From within that random length it selects
/// random start points and gene lengths.
///
/// Upon success it returns JSON with the following schema:
/// ```txt
/// {
///     "length": GENOMIC_REGION_LENGTH,
///     "genes": [
///         (START_IDX1, END_IDX1),
///         (START_IDX2, END_IDX2),
///         (START_IDX3, END_IDX3),
///         ...
///     ]
/// }
/// ```
///
/// # Errors
/// Will return a 500 error if there was a gene generated outside of bounds
#[actix_web::get("/gene")]
async fn gene() -> impl Responder {
    let mut rng = rand::thread_rng();
    let length = rng.gen_range(1000..10000usize);
    let num_genes = rng.gen_range(1..(length / 500));

    let mut genes = vec![];
    let mut start = rng.gen_range(1..100);
    for _ in 0..num_genes {
        let gene_length = rng.gen_range(250..500);
        if start + gene_length >= length {
            return HttpResponse::InternalServerError().finish();
        }
        genes.push((start, start + gene_length));
        start += gene_length;
        start = rng.gen_range(start..cmp::min(start + 250, length));
    }

    HttpResponse::Ok().json(serde_json::json!({
        "length": length,
        "genes": genes
    }))
}
#[actix_web::get("/mol")]
async fn mol() -> impl Responder {
    let mut coordData = vec![];
    let mut bondData = vec![];
    let path1 = env::current_dir();
    let mut file = fs::File::open("mol.json").expect("Can't open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("cannot read file");
    let jsons:JsonValue = serde_json::from_str(&contents).expect("JSON was not well-formatted");  
    for x in 0..8 {
        let v = json!(jsons["atoms"][x]);
        let string = v.to_string();
        let tempJson:JsonValue = serde_json::from_str(&string).expect("JSON was not well-formatted");
        let tempX = json!(tempJson["x"]).to_string();
        let tempY = json!(tempJson["y"]).to_string();
        let mut tempSymbol = json!(tempJson["symbol"]).to_string();
        let mut tempSymbol: &str = &tempSymbol[1..tempSymbol.len() - 1];
        let mut offset = 0;
        if tempSymbol.eq("C"){
            tempSymbol = "";
            offset = 1000;
        } 
        let tempArray = [tempX, tempY, tempSymbol.to_string(), offset.to_string()];
        coordData.push(tempArray);
    }
    for x in 0..8 {
        let v = json!(jsons["bonds"][x]);
        let string = v.to_string();
        let tempJson:JsonValue = serde_json::from_str(&string).expect("JSON was not well-formatted");
        let index = json!(tempJson["atom1"]).to_string();
        let my_int = from_str::<usize>(&index);
        let v = my_int.ok();
        let indexFin1 = v.unwrap();
        let atom1X = &coordData[indexFin1][0];
        let atom1Y = &coordData[indexFin1][1];

        let index2 = json!(tempJson["atom2"]).to_string();
        let my_int2 = from_str::<usize>(&index2);
        let v2 = my_int2.ok();
        let indexFin2 = v2.unwrap();
        let atom2X = &coordData[indexFin2][0];
        let atom2Y = &coordData[indexFin2][1];
        let tempZ = json!(tempJson["mult"]).to_string();
        let init = "0";
        let mut offset = "0";
        if tempZ.eq("2"){
            offset = "5";

        }
        let tempArray = [atom1X, atom1Y, atom2X, atom2Y, init, offset];
        bondData.push(tempArray);
    }
    HttpResponse::Ok().json(serde_json::json!({
        "coordData":  coordData,
        "bondData": bondData
    }))

}

#[actix_web::get("/Bigmol")]
async fn Bigmol() -> impl Responder {
    let mut coordData = vec![];
    let mut bondData = vec![];
    let path1 = env::current_dir();
    let mut file = fs::File::open("bigger_mol.json").expect("Can't open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("cannot read file");
    let jsons:JsonValue = serde_json::from_str(&contents).expect("JSON was not well-formatted");  
    for x in 0..113 {
        let v = json!(jsons["atoms"][x]);
        let string = v.to_string();
        let tempJson:JsonValue = serde_json::from_str(&string).expect("JSON was not well-formatted");
        let tempX = json!(tempJson["x"]).to_string();
        let tempY = json!(tempJson["y"]).to_string();
        let mut tempSymbol = json!(tempJson["symbol"]).to_string();
        let mut tempSymbol: &str = &tempSymbol[1..tempSymbol.len() - 1];
        if tempSymbol.eq("C"){
            tempSymbol = "";
            offset = 1000;
        } 
        let tempArray = [tempX, tempY, tempSymbol.to_string()];
        coordData.push(tempArray);
    }
    for x in 0..117 {
        let v = json!(jsons["bonds"][x]);
        let string = v.to_string();
        let tempJson:JsonValue = serde_json::from_str(&string).expect("JSON was not well-formatted");
        let index = json!(tempJson["atom1"]).to_string();
        let my_int = from_str::<usize>(&index);
        let v = my_int.ok();
        let indexFin1 = v.unwrap();
        let atom1X = &coordData[indexFin1][0];
        let atom1Y = &coordData[indexFin1][1];

        let index2 = json!(tempJson["atom2"]).to_string();
        let my_int2 = from_str::<usize>(&index2);
        let v2 = my_int2.ok();
        let indexFin2 = v2.unwrap();
        let atom2X = &coordData[indexFin2][0];
        let atom2Y = &coordData[indexFin2][1];
        let tempZ = json!(tempJson["mult"]).to_string();
        let init = "0";
        let mut offset = "0";
        if tempZ.eq("2"){
            offset = "10";

        }
        let tempArray = [atom1X.to_string(), atom1Y.to_string(), atom2X.to_string(), atom2Y.to_string(), init.to_string(), offset.to_string()];
        bondData.push(tempArray);
    }

    HttpResponse::Ok().json(serde_json::json!({
        "coordData":  coordData,
        "bondData": bondData
    }))

}
// TODO: add a "/mol" endpoint that r/show/5pwBAjuJJAOt7cED5Lkjnketurns the contents of `mol.json`
// TODO: add a "/bigmol" endpoint that returns the contents of `bigger_mol.json`

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive())
            .wrap(Logger::default())
            .service(mol)
    })
    .bind(("127.0.0.1", 8095))?
    .run()
    .await
    
}

