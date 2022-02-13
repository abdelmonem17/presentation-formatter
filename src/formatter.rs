use serde::{Serialize,Deserialize};
use std::error::Error;
use crate::get_code_from_link;


///green field data to be formatted to presentation
#[derive(Serialize,Deserialize,Default,Debug)]
#[serde(default)]
pub struct UserData{
    pub(crate) title:String,
    pub(crate) text :String,
    pub(crate) sections:Vec<Section>
}

///section that has code and description for this code
#[derive(Serialize,Deserialize,Default,Debug)]
#[serde(default)]
pub struct Section{
    pub(crate) title:String,
    pub(crate) text:String,
    pub(crate) pages:Vec<Page>,
}

///section that has code and description for this code
#[derive(Serialize,Deserialize,Default,Debug)]
#[serde(default)]
pub struct Page{
    pub(crate) title:String,
    pub(crate) code :String,
    pub(crate) text:String,
}

pub trait CodeReader{
    fn read_code(&self,path:&str)->Result<String,Box<dyn Error>>{
        get_code_from_link(path)
    }
}
pub trait FormatReader:{
    fn read(&self,path: &str) ->Result<UserData,Box<dyn Error>>;
}

pub trait FormatWriter{
    fn writer(&self,path: &str,data:&UserData) ->Result<(),Box<dyn Error>>;
}

