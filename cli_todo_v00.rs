// https://docs.rs/structopt/0.3.21/structopt/index.html
// https://blog.logrocket.com/json-and-rust-why-serde_json-is-the-top-choice/
// https://github.com/serde-rs/json
// ✅ ❌ 🚧 🦀


use std::io::BufReader; // Read, Error
use std::path::Path;
use serde::{Deserialize, Serialize};
use serde_json::Result;
// use std::path::PathBuf;
// use structopt::StructOpt;
use std::fs::{ File, write };
use std::env::args;


#[derive(Deserialize, Serialize, Debug)]
struct ItemList {
    items: Vec<Item>,
}

#[derive(Deserialize, Serialize, Debug)]
struct Item {
    title: String,
    content: String,
    status: String,
}

fn create_item(title: String, content: String) -> Item {
    Item {
        title: title,
        content: content,
        status: String::from("❌"),
    }
}

fn remove_item(mut itemlist: ItemList, title: String) -> ItemList {
    for i in 0..itemlist.items.len() {
        if itemlist.items[i].title  == title {
            itemlist.items.remove(i);
        }
    }
    itemlist
}

fn read_itemlist_from_file<P: AsRef<Path>>(path: P) -> Result<ItemList> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path).expect("bad word");
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `ItemList`.
    let u = serde_json::from_reader(reader)?;

    // Return the `ItemList`.
    Ok(u)
}

fn display(itemlist: &ItemList) {
    for i in 0..itemlist.items.len() {
        println!("{}: Title: {} \t|{}|\n{}\n", i, itemlist.items[i].title, itemlist.items[i].status, itemlist.items[i].content);
    }
}

fn change_status(mut itemlist: ItemList, title: String ,mark: String) -> ItemList {
    for i in 0..itemlist.items.len() {
        if itemlist.items[i].title == title {
            itemlist.items[i].status = mark.clone();
        }
    }
    itemlist
}

fn main() {
    let action = args().nth(1).expect("Please specify an action");

    let mut itemlist = read_itemlist_from_file("foo.json").unwrap();

    let checked = "✅".to_string();
    let crossed = "❌".to_string();
    let occupied = "🚧".to_string();

    if action == "add" {
        let item_title = args().nth(2).expect("Please specify item title");
        let item_content = args().nth(3).expect("Please specify item content");
        let item = create_item(item_title, item_content);
        itemlist.items.push(item);
    } else if action == "remove" {
        let item_title = args().nth(2).expect("Please specify item title");
        itemlist = remove_item(itemlist, item_title);
    } else if action == "set-status" {
        let item_title = args().nth(2).expect("Please specify item title");
        let status = args().nth(3).expect("Please specify item status");
        if status == "done" {
            itemlist = change_status(itemlist, item_title, checked.clone());
        } else if status == "undone" {
            itemlist = change_status(itemlist, item_title, crossed.clone());
        } else if status == "occupied" {
            itemlist = change_status(itemlist, item_title, occupied.clone());
        }
    } else if action == "display" {
        display(&itemlist);
    } else {
        println!("HELP");
    }
    

    let j = serde_json::to_string_pretty(&itemlist).expect("bad word");
    write("foo.json", j).expect("bad word");
}


// ???: 1. You should loop over the contents of the array by doing for item in itemlist.items.
//         2. You don't need to do .to_string() on "10", String can be compared against a &str.
// ???: I think this will make it nicer :)


// ???: @??? status = checker.clone() did not solve the problem?

    // for i in 0..itemlist.items.len() {
    //     if itemlist.items[i].title == "10".to_string(){
    //         itemlist.items[i].status = crossed.clone();
    //     }
    // }
    // itemlist.items[0].status = checker;
