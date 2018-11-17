use std::error::Error;
use std::io;
use std::str::FromStr;

pub const FETCH_URI: &'static str = "http://1lo.zory.pl/zas/";
const FETCH_PAGE1_START: &'static str = "<h3>KOMUNIKATY DYREKCJI:</h3><br />";
const FETCH_PAGE1_END: &'static str = "<a name=\"zast\"></a>";
const FETCH_PAGE2_START: &'static str =
    "<div class=\"zastepstwa-container\" data-snap-ignore=\"true\">";
const FETCH_PAGE2_END: &'static str =
    "<SMALL>Przygotowano za pomocą programu firmy <A HREF=\"http://www.vulcan.edu.pl/\" >VULCAN</A></SMALL>";

const AUTOSCROLL: &'static str = include_str!("autoscroll.html");

#[derive(Debug, Clone)]
pub struct ZastepstwaIKomunikaty(pub String, pub String);

impl ZastepstwaIKomunikaty {
    pub fn empty() -> Self {
        ZastepstwaIKomunikaty(String::new(), String::new())
    }
    pub fn fetch() -> Result<Self, Box<Error>> {
        let page = reqwest::get(FETCH_URI)?.text()?;
        let body1_start = page
            .find(FETCH_PAGE1_START)
            .ok_or(io::Error::last_os_error())?
            + FETCH_PAGE1_START.len();
        let body1_end = page
            .find(FETCH_PAGE1_END)
            .ok_or(io::Error::last_os_error())?;
        let body2_start = page
            .find(FETCH_PAGE2_START)
            .ok_or(io::Error::last_os_error())?
            + FETCH_PAGE2_START.len();
        let body2_end = page
            .find(FETCH_PAGE2_END)
            .ok_or(io::Error::last_os_error())?;
        Ok(ZastepstwaIKomunikaty(
            String::from_str(&page[body1_start..body1_end])?,
            String::from_str(&page[body2_start..body2_end])?,
        ))
    }
    pub fn komunikaty(&self) -> String {
        format!(
            "{}{}{}",
            AUTOSCROLL, "<style type=\"text/css\">.zktitle { display: block; color: red; } body { font-size: 13px; }</style>", self.0
        )
    }
    pub fn zastepstwa(&self) -> String {
        format!("{}{}</HTML>", AUTOSCROLL, self.1)
    }
}

impl std::cmp::PartialEq for ZastepstwaIKomunikaty {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}
