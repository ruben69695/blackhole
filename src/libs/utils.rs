pub mod cli {

    pub fn print_help() {
        println!("Blackhole CLI program
    
        USAGE:
            blackhole --help
            blackhole --version
            blackhole <base_path> <interval>
            blackhole --eat <file_path_one> <file_path_two> <directory_path>
            
        EXAMPLE:
            blackhole /home/user/Dowloads
            blackhole /home/user/Downloads 1.5
            blackhole --eat /file.tkt /cat.c /home/ra/directory
            
        OPTIONS:
            --help, -h      print this message
            --version, -v   print current version
            --eat, -e       delete files or directories, if multiple items separate them by spaces
            <base_path>     absolute path where to create the blackhole
            <interval>      [optional] indicates the time in seconds it takes for the black hole to absorb data"
        );
    }

}

pub mod filesystem {
    use std::path::Path;

    pub fn convert_to_paths(str_paths: &[String]) -> Vec<&Path> {

        let mut paths: Vec<&Path> = Vec::new();

        for str_path in str_paths {
            let path: &Path = Path::new(str_path);
            
            if !path.exists() {
                continue;
            }

            paths.push(path);
        }

        return paths;
    }
}
