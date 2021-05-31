pub fn add_one(x: u32) -> u32 {
    return x + 1;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(3, crate::add_one(2));
    }
}
