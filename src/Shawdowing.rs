pub fn shadowing() -> i32 {
    let mut counter: i32 = 0;
    
    let result = loop {
        counter += 1;
        
        if counter == 10{
            break counter;
        }
    };
    
    return result 
}