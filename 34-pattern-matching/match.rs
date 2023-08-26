fn main() {
    #[derive(Debug)]
    enum Language {
        English,
        Spanish,
        Russian,
        Japanese,
    }
    
    let language = Language::English;
    
    match language {
        Language::English => println!("hello world"),
        Language::Spanish => println!("hola mundo"),
        Language::Russian => println!("Russian"),
        lang => println!("None {:?}", lang),
    }
}