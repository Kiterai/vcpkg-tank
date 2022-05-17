use actix_files as web_fs;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::{get, head, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use web_fs::NamedFile;

mod vcpkg;
use vcpkg::*;

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

    let res = vcpkg_start_export(pkgs, uuid.to_string().as_str()).await;

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

    HttpResponse::Accepted().json(VcpkgPrepareResponse {
        id: uuid,
        pkgs: pkgs.to_owned(),
    })
}

#[head("/api/export")]
async fn export_chk(req: web::Query<VcpkgGetRequest>) -> impl Responder {
    let task_state = chk_task_state(&req.id);

    match task_state {
        TaskState::Done => HttpResponse::Ok().finish(),
        TaskState::ErrorOccured => HttpResponse::InternalServerError().finish(),
        TaskState::Progress => HttpResponse::Accepted().finish(),
        TaskState::None => HttpResponse::NotFound().finish(),
    }
}

#[get("/api/export")]
async fn export_get(req: web::Query<VcpkgGetRequest>, req_base: HttpRequest) -> impl Responder {
    let task_state = chk_task_state(&req.id);

    match task_state {
        TaskState::Done => {
            let file = NamedFile::open(get_pkg_file_path(&req.id)).unwrap();

            file.into_response(&req_base)
        }
        TaskState::ErrorOccured => HttpResponse::InternalServerError().finish(),
        TaskState::Progress => HttpResponse::Accepted().finish(),
        TaskState::None => HttpResponse::NotFound().finish(),
    }
}

#[get("/api/export-once")]
async fn export_integrated(req: web::Json<VcpkgPrepareRequest>) -> impl Responder {
    HttpResponse::Ok().finish()
}

#[post("/api/install")]
async fn install(req: web::Json<VcpkgPrepareRequest>) -> impl Responder {
    let res = vcpkg_start_install(req.pkgs.as_slice()).await;

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
            .service(export_chk)
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
