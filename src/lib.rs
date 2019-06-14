mod config;
mod data;

pub use config::DATA;
pub use data::{Data, Entry};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
