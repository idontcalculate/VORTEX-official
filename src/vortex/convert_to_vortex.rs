use std::fs::File;
use std::io::Write;
use prost::Message;
use crate::vortex::{VortexModel, VortexLayer};

// Custom Tensor structure
#[derive(Debug, Clone)]
pub struct Tensor {
    data: Vec<f32>,   // Stores the flattened tensor data
    _shape: Vec<usize>, // Stores the dimensions of the tensor, currently unused
}

impl Tensor {
    // Constructor to create a tensor with given data and shape
    pub fn new(data: Vec<f32>, shape: Vec<usize>) -> Self {
        assert_eq!(data.len(), shape.iter().product(), "Data size must match shape dimensions");
        Tensor { data, _shape: shape }
    }
}

// Conversion function to convert a model into VORTEX format
pub fn convert_to_vortex(
    model_name: &str,
    _model_path: &str,  // Underscore added to suppress the warning
    output_dir: &str,
    vortex_version: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    
    // Example: Creating a dummy layer with custom tensors
    let weights = Tensor::new(vec![0.1, 0.2, 0.3], vec![3, 1]);  // Example weights tensor
    let biases = Tensor::new(vec![0.01, 0.02, 0.03], vec![3]);    // Example biases tensor

    let layer = VortexLayer {
        name: "layer_1".to_string(),
        r#type: "Dense".to_string(),
        input_dim: 768,
        output_dim: 768,
        weights: weights.data.clone(),  // Flattened weights data
        biases: biases.data.clone(),    // Flattened biases data
    };

    let vortex_model = VortexModel {
        name: model_name.to_string(),
        version: vortex_version,
        layers: vec![layer],
    };

    // Serialize and save the model to the VORTEX format
    let output_file_path = format!("{}/{}.vortex", output_dir, model_name);
    let mut file = File::create(&output_file_path)?;
    file.write_all(&vortex_model.encode_to_vec())?;

    println!("Model converted to VORTEX format and saved to {}", output_file_path);

    Ok(())
}
