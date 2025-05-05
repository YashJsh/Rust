
//function takes input as vector and return vector with even values
pub fn even_vector(vec : Vec<i32>)-> Vec<i32>{
    vec.into_iter().filter(|x| x % 2 == 0).collect()
}

pub fn even_vector_method_two(vec : Vec<i32>)-> Vec<i32>{
    let mut vec2 = Vec::new();
    for val in vec{
        if val%2 == 0 {
            vec2.push(val)
        }
    }
    vec2
}  

pub fn pushvector(){
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    vec.push(4);
    vec.push(5);
    println!("{:?}", vec);
}