use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct File {
    name: String,
    size: usize
}

impl File {
    pub fn new(name: &str, size: usize) -> Self {
        Self {
            name: name.to_string(),
            size
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum DirectoryNode {
    File(File),
    Directory(String, String)
}

pub struct CommandLine<'a> {
    directories: Vec<DirectoryNode>,
    working_directory: Vec<&'a str>,
    directory: HashMap<String, Vec<DirectoryNode>>,
}

impl<'a> CommandLine<'a> {
    pub fn new() -> Self {
        Self {
            working_directory: Vec::new(),
            directories: Vec::from([
                DirectoryNode::Directory(
                    "/".to_string(),
                    "/".to_string(),
            )]),
            directory: HashMap::from([
                ("/".to_string(), Vec::new())
            ]),
        }
    }
    pub fn pwd(&self) -> String {
        format!("/{}", self.working_directory.join("/"))
    }
    pub fn cd(&mut self, path: &'a str) {
        match path {
            ".." => if !self.working_directory.is_empty() {
                self.working_directory.pop();
            },
            "/" => self.working_directory = Vec::new(),
            _ => self.working_directory.push(path),
        }
    }
    pub fn add_file(&mut self, filename: &str, size: usize) {
        let file = DirectoryNode::File(File::new(filename, size));
        self.add_node(file);
    }

    pub fn add_directory(&mut self, dir_name: &str) {
        let dir = DirectoryNode::Directory(
            dir_name.to_string(),
        Self::get_dir_path(&self.pwd(), &dir_name)
        );
        self.directories.push(dir.clone());
        self.add_node(dir);
    }

    pub fn ls(&self) -> String {
        match self.directory.get(&self.pwd()) {
            None => String::new(),
            Some(files) => {
                files.iter().map(|file| {
                    match file {
                        DirectoryNode::File(file) =>
                            format!("{} {}", file.size, file.name),
                        DirectoryNode::Directory(name, _) =>
                            format!("dir {}", name),
                    }
                }).collect::<Vec<_>>().join("\n")
            }
        }
    }
    pub fn dir_size(&self, path: &str) -> Option<usize> {
        // Todo memoize this expensive function
        self.directory.get(path)?.iter().fold(0, |accum, dir| {
            match dir {
                DirectoryNode::File(file) => accum + file.size,
                DirectoryNode::Directory(dir_name, _) => {
                    let new_path = Self::get_dir_path(path, &dir_name.as_str());
                    return accum + self.dir_size(new_path.as_str()).unwrap()
                },
            }
        }).into()
    }

    pub fn directories(&self) -> Vec<DirectoryNode> {
        self.directories.clone()
    }

    fn add_node(&mut self, node: DirectoryNode) {
        if let Some(files) = self.directory.get_mut(&self.pwd()) {
            files.push(node);
        } else {
            self.directory.insert(self.pwd(), vec![node]);
        }
    }

    fn get_dir_path(path: &str, dir_name: &&str) -> String {
        let new_path = match path {
            "/" => format!("/{}", dir_name),
            path => format!("{}/{}", path, dir_name),
        };
        new_path
    }
}

#[cfg(test)]
mod command_line_test {
    use crate::cli::{CommandLine, DirectoryNode};
    use crate::parse_filesystem;

    #[test]
    fn it_goes_to_root() {
        let cli = CommandLine::new();
        let pwd = cli.pwd();
        assert_eq!(pwd, "/");
    }

    #[test]
    fn it_doesnt_change_directory_top_of_root() {
        let mut cli = CommandLine::new();
        cli.cd("..");
        let pwd = cli.pwd();
        assert_eq!(pwd, "/");
    }

    #[test]
    fn it_changes_into_directory() {
        let mut cli = CommandLine::new();
        cli.cd("a");
        let pwd = cli.pwd();
        assert_eq!(pwd, "/a");
        cli.cd("b");
        let pwd = cli.pwd();
        assert_eq!(pwd, "/a/b");
        cli.cd("..");
        let pwd = cli.pwd();
        assert_eq!(pwd, "/a");
        cli.cd("/");
        let pwd = cli.pwd();
        assert_eq!(pwd, "/");
    }

    #[test]
    fn it_adds_files() {
        let mut cli = CommandLine::new();
        cli.add_file("filename.txt", 32);
        let output = cli.ls();
        assert_eq!(output, "32 filename.txt");
        cli.add_file("file_2.txt", 44);
        let output = cli.ls();
        assert_eq!(output, "32 filename.txt\n44 file_2.txt");
        cli.cd("/a");
        cli.add_file("a.txt", 22);
        let output = cli.ls();
        assert_eq!(output, "22 a.txt");
        cli.cd("..");
        let output = cli.ls();
        assert_eq!(output, "32 filename.txt\n44 file_2.txt");
    }

    #[test]
    fn it_adds_directory() {
        let mut cli = CommandLine::new();
        cli.add_directory("a");
        let output = cli.ls();
        assert_eq!(output, "dir a");
    }

    #[test]
    fn it_reads_dir_size() {
        let mut cli = CommandLine::new();
        cli.add_file("a.txt", 1);
        assert_eq!(cli.dir_size("/"), Some(1));
        cli.add_file("b.txt", 2);
        assert_eq!(cli.dir_size("/"), Some(3));

        cli.add_directory("c");
        cli.cd("c");
        cli.add_file("d.txt", 1);
        assert_eq!(cli.dir_size("/c"), Some(1));
        assert_eq!(cli.dir_size("/"), Some(4));

        cli.add_directory("e");
        cli.cd("e");
        cli.add_file("f.txt", 30);

        assert_eq!(cli.dir_size("/c/e"), Some(30));
        assert_eq!(cli.dir_size("/"), Some(34));
    }

    #[test]
    fn it_gets_directories() {
        let mut cli = parse_filesystem("$ cd /").unwrap();
        let dirs = cli.directories();
        assert_eq!(dirs.len(), 1);
        assert_eq!(dirs[0], DirectoryNode::Directory("/".to_string(), "/".to_string()));
        cli.add_directory("a");
        let dirs = cli.directories();
        assert_eq!(dirs.len(), 2);
        assert_eq!(dirs[1], DirectoryNode::Directory("a".to_string(), "/a".to_string()));
    }

}
