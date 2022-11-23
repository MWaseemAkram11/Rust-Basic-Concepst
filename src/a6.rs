struct Book{
    pages:i32,ratings:i32
}

fn display_ratings(book: &Book){
    println!("here is book ratings: {}",book.ratings);
}

fn display_pages(book: &Book){
    println!("Here is book pages: {}",book.pages);
}

pub fn main(){
    let book = Book {
        pages:5,ratings:45,
    };

    display_pages(&book);
    display_ratings(&book);
}