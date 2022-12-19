use crate::day7::classes::{Command, Directory, File, CD};
use std::cell::RefCell;
use std::fs;
use std::io::{BufRead, BufReader};
use std::rc::Rc;

pub fn read() {
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
                    let dirs = Directory::new_vec(vec.clone());

                    for d in 0..dirs.len() {
                        let d = Rc::clone(&dirs[d]);
                        d.borrow_mut().set_back(Rc::clone(&dir));
                    }

                    for d in dirs {
                        dir.borrow_mut().add_dir(d);
                    }

                    let files = File::new_vec(vec);

                    for f in files {
                        dir.borrow_mut().add_file(f);
                    }
                }
                Command::Cd(CD::Up) => {
                    if dir.borrow().get_name() != "/" {
                        print!("Up - Befor {:?} ", dir.borrow().get_name());
                        let d = dir.borrow().get_back();
                        dir = Rc::clone(&d);
                        println!("After {:?}", dir.borrow().get_name());
                    }
                }
                Command::Cd(CD::Down(down)) => {
                    print!("Down - Befor {:?} ", dir.borrow().get_name());
                    let index = dir.borrow().contain_dir(&down);

                    match index {
                        Some(index) => {
                            let d = Rc::clone(&dir.borrow().dirs[index]);
                            dir = Rc::clone(&d);
                        }
                        None => {
                            print!("ERROR ");
                            let d = Rc::new(RefCell::new(Directory::new(down)));
                            d.borrow_mut().set_back(Rc::clone(&dir));
                            dir.borrow_mut().add_dir(Rc::clone(&d));
                            dir = Rc::clone(&d);
                        }
                    }

                    println!("After {:?}", dir.borrow().get_name());
                }
            }

            vec = Vec::new();
        }

        vec.push(line);
    }

    loop {
        if dir.borrow().get_name() == "root" {
            break;
        }
        println!(
            "{}: {}",
            dir.borrow().get_name(),
            dir.borrow().get_back().borrow().get_name()
        );
        let d = dir.borrow().get_back();
        dir = Rc::clone(&d);
    }

    println!("Total: {}", dir.borrow().get_len());
    println!("Total: {}", dir.borrow().get_name());
    // println!("{:?}", dir);
}
