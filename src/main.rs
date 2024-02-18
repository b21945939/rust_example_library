struct Book {
    title: &'static str,
    author: &'static str,
    page_count: u16,
}

struct Magazine {
    title: &'static str,
    issue: u8,
    topic: &'static str,
}

enum Publication {
    Book(Book),
    Magazine(Magazine)
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
    //"Kitap: [title] yazar: [author], [page_count] sayfa" ve dergiler için "Dergi: [title] - Sayı: [issue], Konu: [topic]" 
    for item in library.iter() {
        match item {
            Publication::Book(book) => {
                println!("Kitap: {} yazar: {}, {} sayfa", book.title, book.author, book.page_count);
            },
            Publication::Magazine(magazine) => {
                println!("Dergi: {} - Sayı: {}, Konu: {}", magazine.title, magazine.issue, magazine.topic);
            }
        }
    }

    for item in library {
        match item {
            Publication::Book(book) => {
                println!("Kitap: {} yazar: {}, {} sayfa", book.title, book.author, book.page_count);
            },
            Publication::Magazine(magazine) => {
                println!("Dergi: {} - Sayı: {}, Konu: {}", magazine.title, magazine.issue, magazine.topic);
            }
        }
    }
}
