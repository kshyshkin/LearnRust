
struct Book {
    title: String,
    pub author: String,
    pub pages: u32,
}

fn print_book(book :&Book) {
    println!("Title: {}, Author: {}, Pages: {}",
            book.title,
            book.author,
            book.pages);
}

fn set_title(book :&mut Book, title :String) {
    book.title = title;
}

fn add_to_title(book :&mut Book, extra :String) {
    let formated_extra = format!(" ({})", extra);
    book.title.push_str(&formated_extra);
}

fn main() {
    let book = Book {
        title: String::from("Rust"),
        author: String::from("James Gosling"),
        pages: 100,
    };
    // print_book(&book);
    // set_title(&mut book, "Rust Programming".to_string());
    // print_book(&book);
    // add_to_title(&mut book, "Best Book Ever".to_string());
    // print_book(&book);
    book.title = "Rust Programming".to_string();
    print_book(&book);
}