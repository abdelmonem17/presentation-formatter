use crate::{FormatReader, UserData, CodeReader};
use std::error::Error;

pub struct JsonFormat;

impl CodeReader for JsonFormat {

}
impl FormatReader for JsonFormat {
    fn read(&self,path: &str) ->Result<UserData,Box<dyn Error>> {
        let data = std::fs::read_to_string(path)?;
        let data:UserData = serde_json::from_str(data.as_str())?;
        //todo use get_code_from_link() after handling vertical and horizontal
        Ok(data)
    }
}

