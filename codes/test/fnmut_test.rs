
type Result<A> = std::result::Result<A,String>;
type TaskFn = Box<dyn FnMut() -> Result<usize> + Send + Sync>;

fn do_three(mut func: TaskFn) {
    func();
    func();
    let x = func().unwrap();
    assert_eq!(x,7);
}

fn do_twice<F>(mut func: F)
    where F: FnMut()
{
    func();
    func();
}



fn main() 
{

    let mut x: usize = 1;
    
    let add_two_to_x = || x += 2;
    do_twice(add_two_to_x);
    
    assert_eq!(x, 5);
    
    x = 1;
    let add_two_result = move || ->Result<usize> {
        x += 2;
        Ok(x)
    };
    
    do_three(Box::new(add_two_result));
}