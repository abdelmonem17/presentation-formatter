use serde::{Serialize,Deserialize};
use std::error::Error;


///green field data to be formatted to presentation
#[derive(Serialize,Deserialize,Default,Debug)]
#[serde(default)]
pub struct UserData{
    pub(crate) title:String,
    pub(crate) sections:Vec<Section>
}

///section that has code and description for this code
#[derive(Serialize,Deserialize,Default,Debug)]
#[serde(default)]
pub struct Section{
    pub title:String,
    pub introduction:String,
    pub code :String,
    pub details:String,
}

pub trait FormatReader{
    fn read(&self,path: &str) ->Result<UserData,Box<dyn Error>>;
}

pub trait FormatWriter{
    fn writer(&self,path: &str,data:&UserData) ->Result<(),Box<dyn Error>>;
}

pub async fn get_code_from_link(url:&str)->Result<String,Box<dyn Error>>{
   let response =  reqwest::get(url).await?;
    let body = response.bytes().await?;
    Ok(String::from_utf8_lossy(body.as_ref()).to_string())

}


pub fn get_links(data:&str)->Vec<(String,String)> {
    let mut out = Vec::new();
    let regex = regex::Regex::new(r"\[(?P<title>.+)\]\((?P<link>.+)\)").unwrap();
    let matches = regex.captures_iter(data);
    for mat in matches{
        out.push((mat.name("title").unwrap().as_str().to_string()
                  , mat.name("link").unwrap().as_str().to_string()))
    }

    out
}