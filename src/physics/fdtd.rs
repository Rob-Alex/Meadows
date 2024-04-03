/* 
    Robbie Alexander
    Finite Difference Time Domain Simulator
*/
#[derive(Debug)]
struct Point(i32, i32, i32);
impl Point{
    fn new() -> Self {
        Self(0,0,0)
    }
}

#[derive(Debug)]
pub struct Simulator {
    grid: Vec<Point>
}

impl Simulator {
    pub fn new() -> Self {
        Self {
            grid: Vec::new(), 
        }
    }
}