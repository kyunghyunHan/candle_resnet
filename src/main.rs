use candle_core::{DType, Device, Result, Tensor, D};
use candle_resnet::ResNet;

use candle_nn::{
    activation, loss, ops, Conv2d, Dropout, Linear, Module, ModuleT, Optimizer, VarBuilder, VarMap,
};
struct BasicBlock {}
impl BasicBlock {
    fn init() {}

    fn forawrd() {}
}
fn main() {
    let dev = candle_core::Device::cuda_if_available(0).unwrap();

    let mut varmap = VarMap::new();
    let vs = VarBuilder::from_varmap(&varmap, DType::F32, &dev);
   let a=  ResNet::new(&dev, vs);
    println!("{:?}", a);
}
