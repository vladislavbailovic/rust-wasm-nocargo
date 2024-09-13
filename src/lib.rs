#[no_mangle]
pub extern fn add(a: usize, b: usize) -> usize {
    return a+b;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn wat() {
        assert!(5 == add(2,3), "If this fails, we are in trouble");
    }
}
