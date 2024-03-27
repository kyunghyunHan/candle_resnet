pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub struct ResNet{
   test1:i32,
}
impl ResNet {
    pub fn test2()->i32{
        199
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use super::ResNet;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_resnet() {
        let result = ResNet::test2(); // No arguments passed to test2
        assert_eq!(result, 199);
    }
}
