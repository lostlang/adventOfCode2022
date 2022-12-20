use crate::day7::classes::{Command, Directory, File, CD};
use std::cell::RefCell;
use std::fs;
use std::io::{BufRead, BufReader};
use std::rc::Rc;

pub fn read() -> Rc<RefCell<Directory>> {
    let file = fs::File::open("./dataset/day7.txt").unwrap();
    let reader = BufReader::new(file);

    let mut vec: Vec<String> = Vec::new();
    let mut dir = Rc::new(RefCell::new(Directory::new("root".to_string())));

    for line in reader.lines() {
        let line = line.unwrap();

        if line.contains("$ ") && vec.len() > 0 {
            let command = Command::new(&vec);
            match command {
                Command::Ls => {
                    let files = File::new_vec(vec);

                    for f in files {
                        dir.borrow_mut().add_file(f);
                    }
                }
                Command::Cd(CD::Up) => {
                    if dir.borrow().get_name() != "/" {
                        let d = dir.borrow().get_back();
                        dir = Rc::clone(&d);
                    }
                }
                Command::Cd(CD::Down(down)) => {
                    let index = dir.borrow().contain_dir(&down);

                    match index {
                        Some(index) => {
                            let d = Rc::clone(&dir.borrow().dirs[index]);
                            dir = Rc::clone(&d);
                        }
                        None => {
                            let d = Rc::new(RefCell::new(Directory::new(down)));
                            d.borrow_mut().set_back(Rc::clone(&dir));
                            dir.borrow_mut().add_dir(Rc::clone(&d));
                            dir = Rc::clone(&d);
                        }
                    }
                }
            }

            vec = Vec::new();
        }

        vec.push(line);
    }

    loop {
        if dir.borrow().get_name() == "/" {
            break;
        }
        let d = dir.borrow().get_back();
        dir = Rc::clone(&d);
    }

    dir
}
