use jsonschema::JSONSchema;
use std::fs;

fn main() {
    let file_schema =
        fs::read_to_string("../schema.json").expect("Something went wrong reading the schema");
    let file_json =
        fs::read_to_string("../package.json").expect("Something went wrong reading the json");

    //println!("With text:\n{}", file_schema);

    let schema = serde_json::from_str(&file_schema).expect("msg: &str");
    let instance = serde_json::from_str(&file_json).expect("msg: &str");
    let compiled = JSONSchema::compile(&schema).expect("A valid schema");
    let result = compiled.validate(&instance);
    if let Err(errors) = result {
        for error in errors {
            println!("Validation error: {}", error);
            println!("Instance path: {}", error.instance_path);
        }
    }
}
