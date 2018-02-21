struct Node {
    input_count:usize,
    stored:Vec<bool>,
    outdata:Vec<bool>
}
impl Node {
    fn input_to_index(&self,slice: &Vec<bool>) -> usize {
        slice.iter().fold((0,1),|(acc,mul),&bit|(acc+(mul*(1&bit as usize)),mul.wrapping_add(mul))).0
    }

    fn evaluate(&mut self,inputs:Vec<bool>)->bool{
        self.stored=inputs;
        return self.outdata[self.input_to_index(&self.stored)]
    }
    fn new(inputs:usize)-> Node{
        let mut s:Vec<bool>=vec![false;inputs];
        let mut o:Vec<bool>=vec![false;(2 as usize).pow(inputs as u32)];
        return Node{input_count:inputs,stored:s,outdata:o};
    }
}

struct Node3 {
    stored:Vec<bool>,
    outdata:Vec<Vec<Vec<bool>>>
}
impl Node3 {
    fn evaluate(&mut self,inputs:Vec<bool>)->bool{
        self.stored=inputs;
        return self.outdata[self.stored[0] as usize][self.stored[1] as usize][self.stored[2] as usize]
    }
    fn new()-> Node3{
        let mut s:Vec<bool>=vec![false;3];
        let mut o:Vec<Vec<Vec<bool>>>=vec![vec![vec![false;2];2];2];
        return Node3{stored:s,outdata:o};
    }
}
struct Net3 {

}
fn main() {
    println!("Hello, world!");
}
