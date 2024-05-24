pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub struct ResNet {
    in_channles: u32,
    conv1: i32,
    conv2: i32,
    conv3: i32,
}
impl ResNet {
    pub fn new(&mut self) {
        self.in_channles= 64;
    }
}

#[cfg(test)]
mod tests {
    use super::ResNet;
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

}
