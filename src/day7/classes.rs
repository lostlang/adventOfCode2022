use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct File {
    name: String,
    size: u128,
}

impl File {
    pub fn new(name: String, size: u128) -> File {
        File { name, size }
    }

    pub fn new_vec(vec: Vec<String>) -> Vec<Rc<RefCell<File>>> {
        let mut files: Vec<Rc<RefCell<File>>> = Vec::new();

        for f in 1..vec.len() {
            if !vec[f].contains("dir ") {
                let v = vec[f].split(" ").collect::<Vec<&str>>();
                let name = v[1].to_string();
                let size = v[0].parse::<u128>().unwrap();

                files.push(Rc::new(RefCell::new(File::new(name, size))));
            }
        }

        files
    }
}

#[derive(Debug, Clone)]
pub struct Directory {
    name: String,
    pub files: Vec<Rc<RefCell<File>>>,
    pub dirs: Vec<Rc<RefCell<Directory>>>,
    pub back: Option<Rc<RefCell<Directory>>>,
}

impl Directory {
    pub fn new(name: String) -> Directory {
        Directory {
            name,
            files: Vec::new(),
            dirs: Vec::new(),
            back: None,
        }
    }

    pub fn new_vec(vec: Vec<String>) -> Vec<Rc<RefCell<Directory>>> {
        let mut dirs: Vec<Rc<RefCell<Directory>>> = Vec::new();

        for i in 1..vec.len() {
            if vec[i].contains("dir ") {
                let dir = vec[i].replace("dir ", "");
                let dir = Rc::new(RefCell::new(Directory::new(dir.clone())));
                dirs.push(dir);
            }
        }

        dirs
    }

    pub fn add_file(&mut self, file: Rc<RefCell<File>>) {
        self.files.push(file);
    }

    pub fn add_dir(&mut self, dir: Rc<RefCell<Directory>>) {
        self.dirs.push(dir);
    }

    pub fn set_back(&mut self, back: Rc<RefCell<Directory>>) {
        self.back = Some(back);
    }

    pub fn contain_dir(&self, name: &str) -> Option<usize> {
        for file in 0..self.dirs.len() {
            if self.dirs[file].borrow().get_name() == name {
                return Some(file);
            }
        }

        return None;
    }

    pub fn get_len(&self) -> usize {
        self.files.len()
    }

    pub fn get_name(&self) -> &str {
        &self.name.as_str()
    }

    pub fn get_back(&self) -> Rc<RefCell<Directory>> {
        match &self.back {
            Some(back) => Rc::clone(back),
            None => panic!("No back"),
        }
    }
}

#[derive(Debug)]
pub enum Command {
    Ls,
    Cd(CD),
}

impl Command {
    pub fn new(command: &Vec<String>) -> Command {
        let command = command[0].clone();

        if command.contains("ls") {
            Command::Ls
        } else if command.contains("cd") {
            if command.contains("..") {
                Command::Cd(CD::Up)
            } else {
                let command = command.replace("$ cd ", "");
                Command::Cd(CD::Down(command.to_string()))
            }
        } else {
            panic!("Command not found");
        }
    }
}

#[derive(Debug)]
pub enum CD {
    Up,
    Down(String),
}
