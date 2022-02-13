use crate::{FormatWriter, UserData, Section, get_code_from_link, Page};
use std::io::Write;
use std::error::Error;

///
pub struct HackMDFormatter;

impl FormatWriter for HackMDFormatter {
    fn writer(&self,path: &str,data:&UserData) ->Result<(),Box<dyn Error>>{
        let mut file_content = String::new();
        if !data.title.is_empty(){
            file_content = file_content +"# "+ data.title.as_str() + "\n\n";
        }
        if !data.text.is_empty() {
            file_content.push_str(data.text.as_str());
            file_content.push_str("\n\n");
        }

        let mut file = std::fs::File::create(path)?;
        file_content.push_str(&*format_sections_for_hackmd(&data.sections)?);
        file.write_all(file_content.as_bytes())?;
        Ok(())
    }
}

fn format_pages_for_hackmd(pages:&Vec<Page>) ->Result<String,Box<dyn Error>>{
    let mut str = String::new();
    if let Some(first_pg) =pages.get(0){
        str.push_str("---\n\n");
        format_page_for_hackmd(first_pg,&mut str)?
    }
    for i in 1..pages.len(){
        str.push_str("----\n\n");
        str.push('#');
        format_page_for_hackmd(&pages[i],&mut str)?;
    }
    Ok(str)
}
fn format_page_for_hackmd(page:&Page,data:&mut String)->Result<(),Box<dyn Error>>{
    //let mut data = String::new();
    if !page.title.is_empty(){
        *data = data.to_owned() + "## " + page.title.as_str() + "\n\n";
    }
    if !page.code.is_empty(){
        let code = get_code_from_link(page.code.as_str())?;
        if !code.is_empty() {
            *data = data.to_string() + "```rust=\n" + code.as_str() + "\n```\n\n";
        }
    }
    if !page.text.is_empty() {
        data.push_str(page.text.as_str());
        data.push_str("\n\n");
    }
    Ok(())
}
fn format_sections_for_hackmd(sections:&Vec<Section>)->Result<String,Box<dyn Error>>{
    let mut data = String::new();
    for section in sections{
        if !section.title.is_empty(){
            data = data + "## " + section.title.as_str() + "\n\n";
        }
        if !section.text.is_empty(){
            data = data + section.text.as_str() + "\n\n";
        }
        data.push_str(&*format_pages_for_hackmd(&section.pages)?);
    }
    Ok(data)
}