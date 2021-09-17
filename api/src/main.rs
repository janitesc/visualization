use std::cmp;

use actix_cors::Cors;
use actix_web::{middleware::Logger, App, HttpResponse, HttpServer, Responder};
use env_logger::Env;
use rand::Rng;

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

// TODO: add a "/mol" endpoint that returns the contents of `mol.json`
// TODO: add a "/bigmol" endpoint that returns the contents of `bigger_mol.json`

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive())
            .wrap(Logger::default())
            .service(gene)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
