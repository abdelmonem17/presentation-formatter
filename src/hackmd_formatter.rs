use crate::{FormatWriter, UserData, Section, get_code_from_link, Page};
use std::io::Write;
use std::error::Error;

//main page : words 365 at most
//line can hold 53
//we have 13 line in page
//8 lines in main page

const MAIN_PAGE_CHARS:usize = 365;
const LINE_CHARS:usize = 49;
const PAGE_LINES:usize = 12;
//const PAGE_CHARS:usize = 588;
const PAGE_CHARS:usize = 1188;
const PAGE_HEADLINE_LINES:usize = 1;
//const MAIN_PAGE_HEADLINE_LINES:usize = 3;
///hackmd formatter
pub struct HackMDFormatter;

impl FormatWriter for HackMDFormatter {
    fn writer(&self,path: &str,data:&UserData) ->Result<(),Box<dyn Error>>{
        let mut file_content = String::from("<style>
    .present p {
        text-align: left;

    }
</style>\n\n");
        if !data.title.is_empty(){
            file_content = file_content +"# "+ data.title.as_str() + "\n\n";
        }
        if !data.text.is_empty() {
            let mut txt = data.text.as_str();
            let current_idx = MAIN_PAGE_CHARS;

            //to split intro to many pages
            //todo handle special cases like split at in middle of a word
            loop {
                if txt.len() > current_idx {
                    let (current,change) = txt.split_at(current_idx);
                    file_content.push_str(current);
                    file_content.push_str("\n\n---\n\n");

                    txt = change;
                    //eprintln!("main page data must be {} char at most and this equal 8 lines",MAIN_PAGE_CHARS);
                }else{
                    file_content.push_str(txt);
                    file_content.push_str("\n\n");
                    break;
                }

            }
        }

        let mut file = std::fs::File::create(path)?;
        file_content.push_str(&*format_sections_for_hackmd(&data.sections)?);
        file.write_all(file_content.as_bytes())?;
        Ok(())
    }
}

fn format_pages_for_hackmd(pages:&Vec<Page>) ->Result<String,Box<dyn Error>>{
    let mut str = String::new();
    // if let Some(first_pg) =pages.get(0){
    //     //str.push_str("---\n\n");
    //     format_page_for_hackmd(first_pg,&mut str)?
    // }
    for i in 0..pages.len(){
        str.push_str("----\n\n");
        str.push('#');
        format_page_for_hackmd(&pages[i],&mut str)?;
    }
    Ok(str)
}
fn format_page_for_hackmd(page:&Page,data:&mut String)->Result<(),Box<dyn Error>>{
    //let mut data = String::new();
    //write the title
    let title_count = if !page.title.is_empty(){
        *data = data.to_owned() + "## " + page.title.as_str() + "\n\n";
        1
    }else{
        0
    };

    //write the code
    let code_lines_count = if !page.code.is_empty(){
        let (code,(start,end)) = get_code_from_link(page.code.as_str())?;
        if !code.is_empty() {
            *data = data.to_string() + &*format!("```rs={}\n",start) + code.as_str() + "\n```\n\n";
        }
        //lines number
         let lines_counts =  ( (end +1 - start)*5/7)+1;
        if lines_counts > PAGE_LINES{
            PAGE_LINES
        }else {
            lines_counts
        }
    }else{
        0
    };
    //todo handle count of previous lines
    let raw_data_lines_count = if !page.raw_data.is_empty(){
        //let (code,(start,end)) = get_code_from_link(page.code.as_str())?;
            *data = data.to_string() + &*page.raw_data.join("\n") + "\n\n";
        //lines number
        page.raw_data.len()
    }else{
        0
    };
    let used_chars = (code_lines_count + title_count + raw_data_lines_count) * LINE_CHARS  ;
 //   println!("used {:?} ,{} {} {}",used_chars, code_lines_count,title_count,page.text.len());
 //   println!("{:?}",page.text);
    let change_size:isize = PAGE_CHARS as isize - used_chars as isize;
   // let change_size = 350;
    if !page.text.is_empty() {
        if PAGE_CHARS <= used_chars{
            eprintln!("page text with title:{} must be shifted to the next page",page.title)
        }
        else if page.text.len() as isize > change_size{
            eprintln!("page text with title:{} must be {} at most but you enter {}",page.title,change_size,page.text.len());
        }
        data.push_str(page.text.as_str());
        data.push_str("\n\n");
    }
    Ok(())
}
fn format_sections_for_hackmd(sections:&Vec<Section>)->Result<String,Box<dyn Error>>{
    let mut data = String::new();
    for section in sections{
        data.push_str("---\n\n");
        if !section.title.is_empty(){
            data = data + "## " + section.title.as_str() + "\n\n";
        }
        if !section.text.is_empty(){
            let head_chars = if !section.text.is_empty(){
                PAGE_HEADLINE_LINES * LINE_CHARS
            }else{
                0
            };
            let txt_chars:isize = PAGE_CHARS as isize - head_chars as isize;
            if section.text.len() as isize > txt_chars{
                eprintln!("section text with title:'{}' must be {} at most",section.title,txt_chars);
                
            }
            data = data + section.text.as_str() + "\n\n";
        }
        data.push_str(&*format_pages_for_hackmd(&section.pages)?);
    }
    Ok(data)
}