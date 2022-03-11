# presentation-formater

## introduction
used to format input in json or csv or any other dotos (toml,yaml)
to [hackmd](https://hackmd.io/) or any other dotos for presentation. 

## how to use it
1- clone the repo
2- build using    `cargo build --release ,` or install it using `cargo install`
3- run the executable as cli and for help `presentation-formatter --help` 
4- check input.json file to see the josn format that is used.
5- to generate hackmd file format from json use `presentation-formatter --input-format json --input-file ./input.json --output-format hackmd --output-file ./output.hackmd`
`

