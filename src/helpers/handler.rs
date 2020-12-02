use crate::helpers::commands::Cmd;
use crate::helpers::ftp::ResultCode;
use std::ffi::OsString;
use std::fs::File;
use std::fs::{create_dir, remove_dir_all};
use std::io::Read;
use std::path::PathBuf;
use std::path::StripPrefixError;

#[derive(Debug, Clone)]
pub struct Handler {
    pub cwd: PathBuf,
    pub path_root: PathBuf,
}

impl Handler {
    pub fn new() -> Self {
        Handler {
            cwd: PathBuf::from("/"),
            path_root: PathBuf::from("/"),
        }
    }
    pub async fn handle_cmd(&mut self, cmd: Cmd) -> ResultCode {
        match cmd {
            Cmd::Pwd => {
                let msg = format!(
                    "{}",
                    self.cwd
                        .to_str()
                        .unwrap_or("Couldn't get current directory")
                );
                if !msg.is_empty() {
                    let message = format!("{}", msg);
                    println!("{}", message);
                    return ResultCode::PATHNAMECreated;
                }
                ResultCode::FileNotFound
            }
            Cmd::Cwd(directory) => self.cwd(directory).await,
            Cmd::Mkd(directory) => self.mkd(directory).await,
            Cmd::Rmd(path) => self.rmd(path).await,
            Cmd::Unknown(msg) => {
                println!("command not implemented: {}", msg);
                ResultCode::CommandNotImplemented
            }
            _ => ResultCode::CommandNotImplemented,
        }
    }
    async fn cwd(&mut self, directory: PathBuf) -> ResultCode {
        let path = self.cwd.join(&directory);
        let (new, res) = self.complete_path(path);
        self.cwd = new.cwd;
        self.path_root = new.path_root;
        if let Ok(dir) = res {
            self.cwd = dir.to_path_buf();
            println!("dir: {:?}", dir);
            return ResultCode::Ok;
        }
        ResultCode::FileNotFound
    }
    async fn mkd(&mut self, path: PathBuf) -> ResultCode {
        let path = self.cwd.join(&path);
        let parent = get_parent(path.clone());
        if let Some(parent) = parent {
            let parent = parent.to_path_buf();
            let (new, res) = self.complete_path(parent);
            self.cwd = new.cwd;
            self.path_root = new.path_root;
            if let Ok(mut dir) = res {
                if dir.is_dir() {
                    let filename = get_filename(path);
                    if let Some(filename) = filename {
                        dir.push(filename);
                        if create_dir(dir).is_ok() {
                            println!("Folder created successfully");
                            return ResultCode::PATHNAMECreated;
                        }
                    }
                }
            }
        }
        ResultCode::FileNotFound
    }
    async fn rmd(&mut self, path: PathBuf) -> ResultCode {
        let path = self.cwd.join(&path);
        let (new, res) = self.complete_path(path);
        self.cwd = new.cwd;
        self.path_root = new.path_root;
        if let Ok(dir) = res {
            if remove_dir_all(dir).is_ok() {
                println!("Folder successfully removed");
                return ResultCode::RequestedFileActionOkay;
            }
        }
        println!("Couldn't remove folder");
        ResultCode::FileNotFound
    }
    fn complete_path(&self, path: PathBuf) -> (Self, Result<PathBuf, std::io::Error>) {
        let directory = self.path_root.join(if path.has_root() {
            path.iter().skip(1).collect()
        } else {
            path
        });
        let dir = directory.canonicalize();
        if let Ok(ref dir) = dir {
            if !dir.starts_with(&self.path_root) {
                return (
                    self.clone(),
                    Err(std::io::ErrorKind::PermissionDenied.into()),
                );
            }
        } else {
            println!("folder does not exist");
            return (
                self.clone(),
                Err(std::io::ErrorKind::AddrNotAvailable.into()),
            );
        }
        (self.clone(), dir)
    }
}

fn get_parent(path: PathBuf) -> Option<PathBuf> {
    path.parent().map(|p| p.to_path_buf())
}

fn get_filename(path: PathBuf) -> Option<OsString> {
    path.file_name().map(|p| p.to_os_string())
}
