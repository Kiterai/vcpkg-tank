use actix_files as web_fs;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use tokio::process::Command;
use uuid::Uuid;
use web_fs::NamedFile;

#[derive(Deserialize)]
struct VcpkgPrepareRequest {
    pkgs: Vec<String>,
}

#[derive(Serialize)]
struct VcpkgPrepareResponse {
    id: Uuid,
    pkgs: Vec<String>,
}

#[derive(Deserialize)]
struct VcpkgGetRequest {
    id: Uuid,
}

#[derive(Serialize)]
struct VcpkgInstallResponse {
    id: Uuid,
    pkgs: Vec<String>,
}

#[post("/api/export")]
async fn export_request(req: web::Json<VcpkgPrepareRequest>) -> impl Responder {
    let uuid = Uuid::new_v4();
    let pkgs = &req.pkgs;

    // todo!();

    HttpResponse::Accepted().json(VcpkgPrepareResponse {
        id: uuid,
        pkgs: pkgs.to_owned(),
    })
}

#[get("/api/export")]
async fn export_get(req: web::Query<VcpkgGetRequest>) -> impl Responder {
    HttpResponse::Ok().finish()
}

#[get("/api/export-once")]
async fn export_integrated(req: web::Json<VcpkgPrepareRequest>) -> impl Responder {
    HttpResponse::Ok().finish()
}

#[post("/api/install")]
async fn install(req: web::Json<VcpkgPrepareRequest>) -> impl Responder {
    let outdir = "../pkgfiles";

    if !Path::exists(Path::new(outdir)) {
        let res = fs::create_dir(outdir);
        if let Err(_) = res {
            println!("err: creating directory");
            return HttpResponse::InternalServerError().finish();
        }
    }

    let res = Command::new("vcpkg")
        .arg("install")
        .args(&req.pkgs)
        .output()
        .await;

    match res {
        Ok(out) => {
            if out.status.success() {
                println!("{}", String::from_utf8_lossy(&out.stdout));
            } else {
                println!("err: vcpkg");
                println!("{}", String::from_utf8_lossy(&out.stdout));
                return HttpResponse::InternalServerError().finish();
            }
        }
        Err(e) => {
            println!("err: command execution err");
            println!("{}", e.to_string());
            return HttpResponse::InternalServerError().finish();
        }
    }

    let uuid = Uuid::new_v4();

    HttpResponse::Accepted().json(VcpkgInstallResponse {
        id: uuid,
        pkgs: req.pkgs.clone(),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(export_request)
            .service(export_get)
            .service(export_integrated)
            .service(install)
            .service(
                web_fs::Files::new("/", "../frontend/dist")
                    .index_file("index.html")
                    .default_handler(|req: ServiceRequest| {
                        let (http_req, _payload) = req.into_parts();

                        async {
                            let response = NamedFile::open("../frontend/dist/index.html")?
                                .into_response(&http_req);
                            Ok(ServiceResponse::new(http_req, response))
                        }
                    }),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
