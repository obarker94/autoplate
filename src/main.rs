use std::fs::File;
use std::io::Read;
use toml;

#[derive(Debug)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
enum ComponentType {
    atoms,
    molecules,
    organsims,
    templates,
}

#[derive(Debug)]
#[allow(non_snake_case)]
struct Arguments {
    pub component_name: String,
    pub component_type: ComponentType,
}

fn main() {
    let component_name = std::env::args().nth(1).expect("no component name given");
    let component_type = std::env::args().nth(2).expect("no component type given");
    let component_type = match component_type.as_str() {
        "atom" => ComponentType::atoms,
        "molecule" => ComponentType::molecules,
        "organism" => ComponentType::organsims,
        "template" => ComponentType::templates,
        _ => panic!("invalid component type"),
    };
    let args = Arguments {
        component_name,
        component_type,
    };

    let mut react_file_name = args.component_name.clone();
    let first_letter = react_file_name.remove(0).to_uppercase();
    react_file_name.insert(0, first_letter.to_string().chars().next().unwrap());

    let mut contents = String::new();
    let mut file = match File::open("./autoplate.toml") {
        Ok(file) => file,
        Err(_) => {
            println!("No autoplate.toml file found in current working directory.");
            return;
        }
    };
    file.read_to_string(&mut contents).unwrap();

    let value: toml::Value = toml::from_str(&contents).unwrap();

    let location = value
        .get("location")
        .and_then(|location| location.as_str())
        .unwrap_or("./components");

    println!("Location: {}", location);

    std::fs::create_dir_all(format!(
        "{}/{:?}/{}",
        location, args.component_type, react_file_name
    ))
    .unwrap();

    File::create(format!(
        "{}/{:?}/{}/{}.cy.tsx",
        location, args.component_type, react_file_name, react_file_name
    ))
    .unwrap();

    std::fs::write(
        format!(
            "{}/{:?}/{}/{}.cy.tsx",
            location, args.component_type, react_file_name, react_file_name
        ),
        format!(
            "import React from 'react';
import {{{0}}} from './{0}';
             
describe('{0}', () => {{
it('{0} renders correctly', () => {{
    cy.mount(<{0}>Some Text</{0}>);
    cy.get(`[data-cy={1}]`).should(`exist`);
    cy.get(`[data-cy={1}]`).should(`have.class`, `zzz`);
    }});
}})",
            react_file_name, args.component_name
        ),
    )
    .unwrap();

    File::create(format!(
        "{}/{:?}/{}/{}.tsx",
        location, args.component_type, react_file_name, react_file_name
    ))
    .unwrap();

    std::fs::write(
        format!(
            "{}/{:?}/{}/{}.tsx",
            location, args.component_type, react_file_name, react_file_name
        ),
        format!(
            "import React from 'react';

export const {0} = () => {{
    return (
        <div data-cy={{`{1}`}}>
            {1}
        </div>
    );
}}",
            react_file_name, args.component_name
        ),
    )
    .unwrap();

    println!("{:?}", args);
}
