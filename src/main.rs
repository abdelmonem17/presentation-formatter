mod formatter;
mod json_format;
mod hackmd_formater;
mod csv_format;

pub use formatter::*;
pub use json_format::*;
pub use hackmd_formater::*;
pub use csv_format::*;

// fn main() {
//     //let format = JsonFormat;
//     let hackmd = HackMDFormatter;
//     let csv = CSVFormat;
//     let data = csv.read("./input.csv").unwrap();
//     let str = hackmd.writer("./readme.md",&data);
//     println!("{:?}",str);
//     println!("Hello, world!");
// }


use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// input file path
    #[clap(long)]
    input_file:String,
    /// output file path
    #[clap( long,default_value="./output.md")]
    output_file:String,
    /// input file format
    #[clap(long,default_value="json")]
    input_format:String,
    /// output file format
    #[clap(long,default_value="hackmd")]
    output_format:String,
}


fn main() {
    let args = Args::parse();
    let input_format:Box<dyn FormatReader> = match args.input_format.as_str(){
        "json"=>Box::new(JsonFormat),
        "csv"=>Box::new(CSVFormat),
        _=>unimplemented!()
    };
    let formatter:Box<dyn FormatWriter> = match args.output_format.as_str(){
        "hackmd"=>Box::new(HackMDFormatter),
        _=>unimplemented!()
    };
    let data = input_format.read(args.input_file.as_str()).unwrap();
    formatter.writer(args.output_file.as_str(),&data).unwrap();
    println!("done!!");

}