struct Book<'a> {
    title: &'a str,
    author: &'a str,
    page_count: u16,
}

struct Magazine<'a> {
    title: &'a str,
    issue: u8,
    topic: &'a str,
}

enum Publication<'a> {
    Book(Book<'a>),
    Magazine(Magazine<'a>)
}

fn print_publication_list_with_delete<'a>(publications: Vec<Publication>) {
    for publication in publications {
        match publication {
            Publication::Book(book) => {
                println!("Kitap: {} yazar: {}, {} sayfa", book.title, book.author, book.page_count);
            },
            Publication::Magazine(magazine) => {
                println!("Dergi: {} - Sayı: {}, Konu: {}", magazine.title, magazine.issue, magazine.topic);
            }
        }
    }
}

fn print_publication_list<'a>(publications: &Vec<Publication>) {
    for publication in publications {
        match publication {
            Publication::Book(book) => {
                println!("Kitap: {} yazar: {}, {} sayfa", book.title, book.author, book.page_count);
            },
            Publication::Magazine(magazine) => {
                println!("Dergi: {} - Sayı: {}, Konu: {}", magazine.title, magazine.issue, magazine.topic);
            }
        }
    }
}
fn main() {
    // create me a vec<Publication>
    let library: Vec<Publication> = vec![
        Publication::Book(Book {
            title: "The Catcher in the Rye",
            author: "J.D. Salinger",
            page_count: 234,
        }),
        Publication::Magazine(Magazine {
            title: "The Economist",
            issue: 202,
            topic: "Economics",
        })
    ];
    
    print_publication_list(&library);
    print_publication_list_with_delete(library);
}
