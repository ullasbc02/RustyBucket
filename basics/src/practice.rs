struct Area{
    
}

impl Area {
    pub fn new() -> Self {
        Self {}
    }
    pub fn find_area(&self, length: &f64, width: &f64) -> f64 {
        length * width
    }
}
pub fn practice(){
   
let area_calculator = Area {};
let area_calculator2 = Area::new();
// println!("Area of rectangle: {}", area_calculator.find_area(&5.0, &3.0));
// println!("Area of rectangle: {}", area_calculator2.find_area(&5.0, &3.0));
    
}