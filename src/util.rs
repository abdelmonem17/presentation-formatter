use std::error::Error;
//use std::collections::HashMap;
//use std::sync::Mutex;

//use lazy_static::lazy_static;
//use std::cell::RefCell;
use std::io;
use std::io::ErrorKind;
// use std::thread_local;
// thread_local! {
//  pub static CODES:RefCell<HashMap<String,String>> = RefCell::new(HashMap::new());
// }

#[derive(Debug)]
pub struct LinkInfo{
    line_number: String,
    raw_link: String
}
fn get_raw_link_and_line_number(link:&str)->Result<LinkInfo,Box<dyn Error>>{
    //old
    //https://github.com/abdelmonem17/starter-rs/main/Cargo.toml
    //new
    //https://raw.githubusercontent.com/abdelmonem17/starter-rs/main/Cargo.toml
    let link = link.replace("https://github.com/","https://raw.githubusercontent.com/");
    let hash_position = link.rfind("#")
        .ok_or(std::io::Error::new(ErrorKind::InvalidData
                                   ,"the url format must be like: https://github.com/path-to-file#line_number"))?;
    let (raw_link,line_number,) = link.split_at(hash_position);
    Ok(LinkInfo{ line_number:line_number.replace('#',""),raw_link:raw_link.replace("/blob","") })
}

///get line of codes that determined in the url and (start,end) of the lines
pub fn get_code_from_link(url:&str) ->Result<(String,(usize,usize)),Box<dyn Error>>{
    let info = get_raw_link_and_line_number(url)?;
  //  println!("{:?}",info);
    //todo use async version
    let response =  reqwest::blocking::get(info.raw_link.as_str())?;
    let body = response.bytes()?;
    let code = String::from_utf8_lossy(body.as_ref());
    let (lines,lines_info) = get_lines_from_code(code.as_ref(),info.line_number.as_str())?;
    Ok((lines,lines_info))
}

///returns (code,(start_line_number,end))
fn get_lines_from_code<'s>(code:&'s str,code_line:&str)->Result<(String,(usize,usize)),Box<dyn Error>>{
    let lines = code_line.split('-').map(|st|
        st.trim_start_matches('L')
            .parse()
            .map_err(|_e| io::Error::new(ErrorKind::InvalidData,format!("can't parse line number {}",st)) )

    ).collect::<Result<Vec<usize>,io::Error>>()?;
    let end = lines.get(1).unwrap_or(&lines[0]);
   Ok((get_line_by_numbers(code,lines[0],*end)?,(lines[0],*end) ))
}

fn get_line_by_numbers(code:&str,start:usize,end:usize)->Result<String,Box<dyn Error>>{
    let lines:String = code.split('\n')
        .skip(start - 1).take(end - start + 1)
        .collect::<Vec<_>>().join("\n");
    Ok(lines)
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