pub struct Port;

impl Port {
    pub fn find_port_argument_or_get_default(args: &Vec<String>) -> usize {
        let default_port = 8080usize;
        let port_args: Vec<String> = args
            .clone()
            .into_iter()
            .filter(|arg| arg.contains("--port="))
            .collect();
        if port_args.len() == 0 {
            default_port
        } else {
            let first_argument = port_args[0].clone().replace("--port=", "");
            match first_argument.parse::<usize>() {
                Ok(port) => port,
                Err(_) => default_port,
            }
        }
    }
}
