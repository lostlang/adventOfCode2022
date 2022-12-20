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

    pub fn new_vec(vec: Vec<String>) -> Vec<File> {
        let mut files: Vec<File> = Vec::new();

        for f in 1..vec.len() {
            if !vec[f].contains("dir ") {
                let v = vec[f].split(" ").collect::<Vec<&str>>();
                let name = v[1].to_string();
                let size = v[0].parse::<u128>().unwrap();

                files.push(File::new(name, size));
            }
        }

        files
    }
}

#[derive(Debug, Clone)]
pub struct Directory {
    name: String,
    pub files: Vec<File>,
    pub dirs: Vec<Rc<RefCell<Directory>>>,
    pub size: u128,
    pub back: Option<Rc<RefCell<Directory>>>,
}

impl Directory {
    pub fn new(name: String) -> Directory {
        Directory {
            name,
            files: Vec::new(),
            dirs: Vec::new(),
            size: 0,
            back: None,
        }
    }

    pub fn add_file(&mut self, file: File) {
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
        self.dirs.len()
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

    pub fn size_update(&mut self) {
        let mut size = 0;

        for file in &self.files {
            size += file.size;
        }

        for dir in &self.dirs {
            if dir.borrow().size == 0 {
                dir.borrow_mut().size_update();
            }
            size += dir.borrow().size;
        }

        self.size = size;
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
        let command = command.split(" ").collect::<Vec<&str>>();

        if command[1] == "ls" {
            return Command::Ls;
        } else if command[1] == "cd" {
            if command[2] == ".." {
                return Command::Cd(CD::Up);
            } else {
                return Command::Cd(CD::Down(command[2].to_string()));
            }
        } else {
            panic!("Invalid command");
        }
    }
}

#[derive(Debug)]
pub enum CD {
    Up,
    Down(String),
}
