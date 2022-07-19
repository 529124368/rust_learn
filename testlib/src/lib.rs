pub mod factorey;
#[cfg(test)]
mod tests {
    use crate::factorey;
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    #[test]
    fn test01() {
        factorey::fac::prt();
    }
}
