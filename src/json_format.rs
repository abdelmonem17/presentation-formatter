use crate::{FormatReader, UserData};
use std::error::Error;

pub struct JsonFormat;

impl FormatReader for JsonFormat {
    fn read(&self,path: &str) ->Result<UserData,Box<dyn Error>> {
        let data = std::fs::read_to_string(path)?;
        let data:UserData = serde_json::from_str(data.as_str())?;
        Ok(data)
    }
}

