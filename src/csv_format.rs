use crate::{FormatReader, UserData};
use std::error::Error;
use crate::Section;

pub struct CSVFormat;

impl FormatReader for CSVFormat{
    fn read(&self, path: &str) -> Result<UserData, Box<dyn Error>> {
        let mut rdr = csv::Reader::from_reader(std::fs::File::open(path)?);
        let mut data = UserData{ title: "".to_string(), sections: vec![] };

        for result in rdr.deserialize() {
            // Notice that we need to provide a type hint for automatic
            // deserialization.
            let section: Section = result?;
            data.sections.push(section);
            //println!("{:?}", record);
        }

        Ok(data)
    }
}