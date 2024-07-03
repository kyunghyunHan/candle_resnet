use candle_core::{Device, Tensor};
use candle_nn::linear::Linear;
use candle_nn::{conv2d, Conv2d, VarBuilder};
#[derive(Debug)]
pub struct ResNet {
    cn1: Conv2d,
    cn2: Conv2d,
    fc1: Linear,
    fc2: Linear,
    fc3: Linear,
}
impl ResNet {
    pub fn new(device: &Device, vs: VarBuilder) -> Self {
        let cn1 = conv2d(3, 6, 5, Default::default(), vs.pp("c1")).unwrap();
        let cn2 = conv2d(6, 16, 5, Default::default(), vs.pp("c2")).unwrap();
        let fc1 = Linear::new(
            Tensor::new(&[[16 * 5 * 5i64]], device).unwrap(),
            Some(Tensor::new(&[84i64], device).unwrap()),
        );
        let fc2 = Linear::new(
            Tensor::new(&[[120i64]], device).unwrap(),
            Some(Tensor::new(&[84i64], device).unwrap()),
        );
        let fc3 = Linear::new(
            Tensor::new(&[[84i64]], device).unwrap(),
            Some(Tensor::new(&[10i64], device).unwrap()),
        );
        Self {
            cn1,
            cn2,
            fc1,
            fc2,
            fc3,
        }
    }
}
