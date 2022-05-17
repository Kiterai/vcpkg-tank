use std::path::PathBuf;
use tokio::process::Command;
use uuid::Uuid;

const PKGDIR_PATH: &str = "../pkgfiles";

pub enum TaskState {
    Progress,
    ErrorOccured,
    Done,
    None,
}

pub fn get_progress_log_path(id: &Uuid) -> PathBuf {
    PathBuf::from(format!("{}/{}.progress.log", PKGDIR_PATH, id.to_string()).as_str())
}

pub fn get_pkg_file_path(id: &Uuid) -> PathBuf {
    PathBuf::from(format!("{}/{}.zip", PKGDIR_PATH, id.to_string()).as_str())
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
    pkgs: &[String],
    output_file: &str,
) -> Result<std::process::Output, std::io::Error> {
    Command::new("vcpkg")
        .arg("export")
        .args(pkgs)
        .arg("--zip")
        .arg(format!("--output-dir={}", PKGDIR_PATH))
        .arg(format!("--output={}", output_file))
        .output()
        .await
}

pub async fn vcpkg_start_install(pkgs: &[String]) -> Result<std::process::Output, std::io::Error> {
    Command::new("vcpkg")
        .arg("install")
        .args(pkgs)
        .output()
        .await
}
