// toml config
use std::fmt;
use std::fs;
use toml::Value;
use substring::*;

pub fn load_config() -> Config {
    let config_file: String = fs::read_to_string("config.toml").expect("Error reading da file");
    let config_obj: Value = toml::from_str(&config_file).expect("Issue deserializing the config file");

    Config {
        name: String::from(config_obj["name"].as_str().expect("No name specified in config.toml")),
        class: String::from(config_obj["class"].as_str().expect("No class specifid in config.toml")),
        language: String::from(config_obj["language"].as_str().expect("No language specified in config.toml")),
        ide: String::from(config_obj["ide"].as_str().expect("No IDE specified in config.toml"))
    }
}

// I probably don't need this, but I included it anyways
pub struct Config {
    pub name: String,
    pub class: String,
    pub language: String,
    pub ide: String
}

pub struct Header {
    name: String,
    class: String,
    helper: String,
    program_name: String, 
    due_date: String,
    language: String,
    ide: String,
    purpose: String, 
    bugs: String,
}

// i wanna do this better by extending display
impl Header {
    pub fn new(config: Config, helper: String, program_name: String, due: String, purpose: String, bugs: String) -> Header {
        println!("CREATING NEW THING");
        Header {
            name:config.name,
            class:config.class, 
            helper: helper.trim().to_string(),
            program_name: program_name.trim().to_string(),
            due_date: due.trim().to_string(),
            language:config.language,
            ide:config.ide,
            purpose:purpose.trim().to_string(),
            bugs:bugs.trim().to_string()
        }
    }
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        let helper_string = gen_var_len_str("Helpers: ".to_owned() + &self.helper);
        let purpose_string = gen_var_len_str("Purpose: ".to_owned() + &self.purpose);
        let bugs_string = gen_var_len_str("Bugs: ".to_owned() + &self.bugs);

        write!(f, "{:#<84}\n# {:<80} #\n# {:<80} #\n# {:<80} #\n{}# {:<80} #\n# {:<80} #\n# {:<80} #\n# {:<80} #\n# {:<80} #\n# {:<80} #\n{}# {:<80} #\n{}{:#<84}", 
            "",
            "Alias: ".to_owned() + &(self.name.as_str()), 
            "Class: ".to_owned() + &(self.class.as_str()), 
            "",
            &(helper_string.as_str()), 
            "",
            "Program: ".to_owned() + &(self.program_name.as_str()), 
            "Due Date: ".to_owned() + &(self.due_date.as_str()), 
            "Language: ".to_owned() + &(self.language.as_str()),
            "IDE: ".to_owned() + &(self.ide.as_str()),
            "",
            &(purpose_string.as_str()),
            "",
            &(bugs_string.as_str()),
            ""
        )
    }
}

fn gen_var_len_str(input: String) -> String {
    let mut final_string = String::new();
    let mut count = 0;

    for _ in 0..(input.chars().count() / 80 + 1) {
        final_string += format!("# {:<80} #\n", input.substring(count * 80,count * 80 + 80)).as_str();
        count += 1;
    }

    final_string
}
