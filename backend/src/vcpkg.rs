use actix::prelude::*;
use std::{
    fs::{self, File},
    path::PathBuf,
    process::{Command, Output},
};
use uuid::Uuid;

const PKGDIR_PATH: &str = "../pkgfiles";

pub struct VcpkgActor;

#[derive(Message)]
#[rtype(result = "Result<Output, std::io::Error>")]
pub struct ExportRequest {
    id: Uuid,
    pkgs: Vec<String>,
}

#[derive(Message)]
#[rtype(result = "Result<Output, std::io::Error>")]
pub struct InstallRequest {
    id: Uuid,
    pkgs: Vec<String>,
}

impl Actor for VcpkgActor {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Context<Self>) {}
    fn stopped(&mut self, _ctx: &mut Context<Self>) {}
}

impl Handler<ExportRequest> for VcpkgActor {
    type Result = Result<Output, std::io::Error>;

    fn handle(&mut self, msg: ExportRequest, _ctx: &mut Context<Self>) -> Self::Result {
        let res = File::create(get_progress_log_path(&msg.id));

        if let Err(e) = res {
            println!("file creation error: {}", e.to_string());
            return Err(e);
        }

        let res = Command::new("vcpkg")
            .arg("export")
            .args(msg.pkgs)
            .arg("--zip")
            .arg(format!("--output-dir={}", get_pkg_file_dir_path(&msg.id)))
            .arg(format!("--output={}", get_pkg_file_name()))
            .output();

        match res {
            Err(e) => {
                println!("command execution error: {}", e.to_string());
                Err(e)
            }
            Ok(out) => {
                let res = fs::remove_file(get_progress_log_path(&msg.id));

                if let Err(e) = res {
                    println!("file removing error: {}", e.to_string());
                    return Err(e);
                }

                Ok(out)
            }
        }
    }
}

impl Handler<InstallRequest> for VcpkgActor {
    type Result = Result<Output, std::io::Error>;

    fn handle(&mut self, msg: InstallRequest, _ctx: &mut Context<Self>) -> Self::Result {
        let res = Command::new("vcpkg").arg("install").args(msg.pkgs).output();
        match res {
            Err(e) => {
                println!("command execution error: {}", e.to_string());
                Err(e)
            }
            Ok(out) => {
                Ok(out)
            }
        }
    }
}

pub enum TaskState {
    Progress,
    ErrorOccured,
    Done,
    None,
}

pub fn get_progress_log_path(id: &Uuid) -> PathBuf {
    PathBuf::from(format!("{}/{}.progress.log", PKGDIR_PATH, id.to_string()).as_str())
}

pub fn get_pkg_file_dir_path(id: &Uuid) -> String {
    format!("{}/out{}", PKGDIR_PATH, id.to_string())
}

pub fn get_pkg_file_name() -> String {
    "pkgfile".to_owned()
}

pub fn get_pkg_file_path(id: &Uuid) -> PathBuf {
    PathBuf::from(format!("{}/{}.zip", get_pkg_file_dir_path(id), get_pkg_file_name()).as_str())
}

pub fn get_error_log_path(id: &Uuid) -> PathBuf {
    PathBuf::from(format!("{}/{}.error.log", PKGDIR_PATH, id.to_string()).as_str())
}

pub fn chk_task_state(id: &Uuid) -> TaskState {
    let progress_log_path = get_progress_log_path(id);
    let pkg_file_path = get_pkg_file_path(id);
    let err_log_path = get_error_log_path(id);

    let is_progress = progress_log_path.exists();
    let is_err_occured = err_log_path.exists();
    let is_valid =
        !is_progress && !is_err_occured && pkg_file_path.exists() && pkg_file_path.is_file();

    if is_valid {
        TaskState::Done
    } else if is_err_occured {
        TaskState::ErrorOccured
    } else if is_progress {
        TaskState::Progress
    } else {
        TaskState::None
    }
}

pub async fn vcpkg_start_export(
    addr: &Addr<VcpkgActor>,
    pkgs: &[String],
) -> Result<Uuid, std::io::Error> {
    let id = Uuid::new_v4();

    actix_web::rt::spawn(addr.send(ExportRequest {
        id: id,
        pkgs: pkgs.to_vec(),
    }));

    Ok(id)
}

pub async fn vcpkg_start_install(
    addr: &Addr<VcpkgActor>,
    pkgs: &[String],
) -> Result<Uuid, std::io::Error> {
    let id = Uuid::new_v4();

    actix_web::rt::spawn(addr.send(InstallRequest {
        id: id,
        pkgs: pkgs.to_vec(),
    }));

    Ok(id)
}

pub async fn vcpkg_export(
    _addr: &Addr<VcpkgActor>,
    pkgs: &[String],
) -> Result<Uuid, std::io::Error> {
    let id = Uuid::new_v4();

    File::create(get_progress_log_path(&id))?;

    let _out = Command::new("vcpkg")
        .arg("export")
        .args(pkgs)
        .arg("--zip")
        .arg(format!("--output-dir={}", get_pkg_file_dir_path(&id)))
        .arg(format!("--output={}", get_pkg_file_name()))
        .output()?;

    fs::remove_file(get_progress_log_path(&id))?;

    Ok(id)
}
