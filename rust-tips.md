
# Rust Tips


## println! & cargo test

    cargo test -- --nocapture


## str <--> String

    // str & String in rust
    // http://hermanradtke.com/2015/05/06/creating-a-rust-function-that-accepts-string-or-str.html
    // http://hermanradtke.com/2015/05/29/creating-a-rust-function-that-returns-string-or-str.html
    // http://doc.rust-lang.org/std/primitive.str.html
        str -> String
        　　a_str.to_string()

        String -> str
        　　&a_string
        　　&a_string[..]


## enums, match

    http://blog.rust-lang.org/2015/04/17/Enums-match-mutation-and-moves.html

    http://www.rustup.com/rust-matching-mutation-moves-za-tan.html



## ffi

    http://hermanradtke.com/2016/03/17/unions-rust-ffi.html