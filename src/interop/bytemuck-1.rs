use crate::*;

unsafe impl bytemuck::Zeroable          for HWND {} // bytemuck = "1.0.0"
unsafe impl bytemuck::Pod               for HWND {} // bytemuck = "1.0.0"

unsafe impl bytemuck::ZeroableInOption  for NonNullHWND {} // bytemuck = "1.10.0"
unsafe impl bytemuck::PodInOption       for NonNullHWND {} // bytemuck = "1.10.0"
