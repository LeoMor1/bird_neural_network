use rand::Rng;

use super::layer::Layer;

#[derive(Debug)]
pub struct Network {
    layers: Vec<Layer>,
}

#[derive(Debug)]
pub struct LayerToplology {
    pub neurons: usize,
}

impl Network {
    pub fn random(rng: &mut dyn Rng ,layers: &[LayerToplology]) -> Self {
        assert!(layers.len() > 1);
        let layers = layers
            .windows(2)
            .map(|layers| Layer::random(rng, layers[0].neurons, layers[1].neurons))
            .collect();
        Self { layers }
    }

    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        // layers = niveaux de reseaux donc layer 1 calculer et donne sont resultat a layer 2
        // la boucle simule ce transfers de valeurs
        self.layers
            .iter()
            .fold(inputs, |inputs, layer| layer.propagate(inputs))
    }
}
