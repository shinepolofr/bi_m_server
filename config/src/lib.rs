pub mod cfgs;
pub mod get_config;

pub use get_config::CFG;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = &CFG.database;
        assert!(!String::is_empty(&result.url));
    }
}
