use crate::{FormatWriter, UserData, Section};
use std::io::Write;
use std::error::Error;

///
pub struct HackMDFormatter;

impl FormatWriter for HackMDFormatter {
    fn writer(&self,path: &str,data:&UserData) ->Result<(),Box<dyn Error>>{
        let mut file_content = String::new();
        if !data.title.is_empty(){
            file_content = file_content +"# "+ data.title.as_str() + "\n";
        }

        let mut file = std::fs::File::create(path)?;
        file_content.push_str(&*format_sections_for_hackmd(&data.sections));
        file.write_all(file_content.as_bytes())?;
        Ok(())
    }
}

fn format_sections_for_hackmd(sections:&Vec<Section>)->String{
    let mut data = String::new();
    for section in sections{
        if !section.title.is_empty(){
            data = data + "## " + section.title.as_str() + "\n";
        }
        if !section.introduction.is_empty(){
            data = data + section.introduction.as_str() + "\n";
        }

        if !section.code.is_empty(){
            data = data +"```rust=\n"+section.code.as_str() + "\n```\n";
        }
        if !section.details.is_empty(){
            data = data + section.details.as_str() + "\n";
        }
    }
    data
}