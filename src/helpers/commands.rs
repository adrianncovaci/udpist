use crate::helpers::error::Error;
use std::path::{Path, PathBuf};
use std::str::FromStr;

#[derive(Clone, Debug)]
pub enum Cmd {
    Cwd(PathBuf),
    Mkd(PathBuf),
    Pwd,
    Rmd(PathBuf),
    Unknown(String),
}

impl AsRef<str> for Cmd {
    fn as_ref(&self) -> &str {
        match *self {
            Cmd::Cwd(_) => "CWD",
            Cmd::Pwd => "PWD",
            Cmd::Mkd(_) => "MKD",
            Cmd::Rmd(_) => "RMD",
            Cmd::Unknown(_) => "UNKN",
        }
    }
}

impl Cmd {
    pub fn new(input: Vec<u8>) -> Result<Cmd, Error> {
        let mut iter = input.split(|&byte| byte == b' ');
        let mut cmd = iter
            .next()
            .ok_or_else(|| Error::Msg("empty command".to_string()))?
            .to_vec();
        to_uppercase(&mut cmd);
        let data = iter
            .next()
            .ok_or_else(|| Error::Msg("no attributes provided".to_string()));
        let cmd = match &cmd[..] {
            b"CWD" => Cmd::Cwd(
                data.and_then(|bytes| Ok(Path::new(std::str::from_utf8(bytes)?).to_path_buf()))?,
            ),
            b"PWD" => Cmd::Pwd,
            b"MKD" => Cmd::Mkd(
                data.and_then(|bytes| Ok(Path::new(std::str::from_utf8(bytes)?).to_path_buf()))?,
            ),
            b"RMD" => Cmd::Rmd(
                data.and_then(|bytes| Ok(Path::new(std::str::from_utf8(bytes)?).to_path_buf()))?,
            ),
            s => Cmd::Unknown(std::str::from_utf8(s).unwrap_or("").to_owned()),
        };
        Ok(cmd)
    }
}

fn to_uppercase(data: &mut [u8]) {
    for byte in data {
        if *byte >= 'a' as u8 && *byte <= 'z' as u8 {
            *byte -= 32;
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum TransferType {
    Ascii,
    Image,
    Unknown,
}

impl From<u8> for TransferType {
    fn from(c: u8) -> TransferType {
        match c {
            b'A' => TransferType::Ascii,
            b'I' => TransferType::Image,
            _ => TransferType::Unknown,
        }
    }
}
