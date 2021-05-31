pub fn add_two(x: u32) -> u32  {
    return x + 2;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(4, crate::add_two(2));
    }
}
