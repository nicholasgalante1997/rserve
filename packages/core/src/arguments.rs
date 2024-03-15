use std::env;

/// # Arguments
///
/// A functional struct that provides associated methods
/// for working with command line arguments
///
pub struct Arguments;

impl Arguments {
    ///
    /// Utility function over env::args().collect()
    ///
    pub fn get_command_line_args() -> Vec<String> {
        env::args().collect()
    }

    ///
    /// Provided a string slice, it will filter the result of
    /// Self::get_command_line_args to see which command line arguments
    /// contain the provided string
    ///
    /// Since its intended usage is to select variable command line arguments
    /// based on a flag, this method strips the provided pattern from the vec of matches
    /// that it returns.
    ///
    /// i.e.
    ///
    /// ```
    /// $ cargo run -- --port=3000
    ///
    /// ...
    ///
    /// Arguments::search_cli_args_on_pattern("--port=");
    /// => ["3000"]
    /// ```
    ///
    /// ### Intended Usage
    ///
    /// ```
    /// let port_args = Arguments::search_cli_args_on_pattern("--port=");
    /// ```
    pub fn search_cli_args_on_pattern(flag_pattern: &str) -> Vec<String> {
        let args = Self::get_command_line_args();
        args.into_iter()
            .filter(|arg| arg.contains(&flag_pattern))
            .map(|arg| arg.replace(flag_pattern, ""))
            .collect()
    }

    pub fn find_directory_arguments() -> Vec<String> {
        Self::search_cli_args_on_pattern("--dir=")
    }

    pub fn find_port_argument_or_get_default() -> usize {
        let default_port = 8080usize;
        let port_args: Vec<String> = Self::search_cli_args_on_pattern("--port=");
        if port_args.len() == 0 {
            default_port
        } else {
            let first_argument = port_args
                .get(0)
                .expect("Proved port_args.len() is at least 1.");
            match first_argument.parse::<usize>() {
                Ok(port) => port,
                Err(_) => default_port,
            }
        }
    }

    pub fn find_host_argument_or_get_default() -> String {
        let default_host = String::from("127.0.0.1");
        let host_args: Vec<String> = Self::search_cli_args_on_pattern("--host=");
        if host_args.len() == 0 {
            default_host
        } else {
            host_args
                .get(0)
                .expect("Proved host_args.len() is at least 1.")
                .to_owned()
        }
    }

    pub fn find_cors_argument_or_get_default() -> Option<String> {
        let cors_arguments = Self::search_cli_args_on_pattern("--cors=");

        if cors_arguments.len() == 0 {
            return None;
        }

        if cors_arguments.len() > 0 {
            let cors_argument = cors_arguments
                .get(0)
                .expect("Proved cors_arguments.len() is at least 1.");
            if cors_argument.as_str() == "true" || cors_argument.as_str() == "*" {
                return Some(String::from("*"));
            }

            return Some(cors_argument.to_owned());
        }

        None
    }

    pub fn find_compression_argument_or_get_default() -> Option<()> {
        let compression_arguments = Self::search_cli_args_on_pattern("--no-compression");
        if compression_arguments.len() > 0 {
            Some(());
        }

        None
    }

    pub fn find_cache_control_argument_or_get_default() -> String {
        let cache_control_default = String::from("private, max-age=259200, must-revalidate");
        let cache_control_arguments = Self::search_cli_args_on_pattern("--cache-control=");
        if cache_control_arguments.len() > 0 {
            let supplied_cache_time = cache_control_arguments
                .get(0)
                .expect("Proved cache control args len() is greater than 0.")
                .to_owned();

            return format!("private, max-age={}, must-revalidate", supplied_cache_time);
        }

        cache_control_default
    }
}
