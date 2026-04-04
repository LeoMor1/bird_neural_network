#[derive(Debug)]
pub struct Network {
    layers: Vec<Layer>,
}

#[derive(Debug)]
pub struct Layer;

impl Network {
    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        todo!()
    }
}
