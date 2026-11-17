use std::fs;
use std::io::Write;

enum Status {
    Pendente,
    Concluido
}

#[derive(Debug, PartialEq)]
pub struct Config {
    command: String,
    task_name: String
}

impl Config {
    pub fn parse_config(args : &Vec<String>) -> Result<Config, &'static str> {

        if args.len() != 2 {
            return Err("Type the COMMAND and TASK NAME!");
        }

        let command: String = args[0].clone();
        let task_name: String = args[1].clone();

        Ok(Config { command, task_name })
    }
}

pub fn exe_command(config: &Config) -> Result<fs::File, &'static str> { 
    
    let mut arq = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("/home/ryu/cli-todo-list.txt") // deixar o home para o user final
        .expect("Something went wrong opening the file!");

    writeln!(arq, "{0} {1}", config.command, config.task_name).expect("writeln failed!");

    return Ok(arq);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_config_less_args() {
        let v: Vec<String> = vec![String::from("add"), String::from("add")];
        assert_eq!(Config::parse_config(&v), Ok(Config { command :String::from("add"), task_name: String::from("add") }));
    }
}