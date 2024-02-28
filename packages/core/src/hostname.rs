pub struct Hostname;

impl Hostname {
    pub fn find_host_argument_or_get_default(args: &Vec<String>) -> String {
        let default_host = String::from("127.0.0.1");
        let host_args: Vec<String> = args
            .clone()
            .into_iter()
            .filter(|arg| arg.contains("--host="))
            .collect();
        if host_args.len() == 0 {
            default_host
        } else {
            host_args[0].clone().replace("--host=", "")
        }
    }
}
