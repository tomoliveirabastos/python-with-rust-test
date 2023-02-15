pub mod pessoa {
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Pessoa<T> {
        pub nome: String,
        pub idade: T,
    }

    impl<T> Pessoa<T> {
        pub fn new(nome: String, idade: T) -> Pessoa<T> {
            return Pessoa { nome, idade };
        }
    }
}
