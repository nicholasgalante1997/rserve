pub struct DefaultFile;

impl DefaultFile {
    pub fn find_default_file_arguments(args: &Vec<String>) -> Vec<String> {
        args.clone()
            .into_iter()
            .filter(|arg| arg.contains("--default-file-path="))
            .collect()
    }
}

impl DefaultFile {
    pub fn get_default_file_or_default(args: &Vec<String>) -> String {
        let default_file_args = DefaultFile::find_default_file_arguments(args);
        if default_file_args.len() > 0 {
            default_file_args[0].clone()
        } else {
            String::from("403.html")
        }
    }
}
