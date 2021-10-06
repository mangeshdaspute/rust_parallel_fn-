use rayon::prelude::*;
fn main() {
//let filenames=String::from("a.fits");
let filenames=["a".to_string(),"b".to_string()];//let filenames=&paths;
let mut vec1:Vec<f64>=Vec::new();
vec1=filenames.par_iter().map(|filename| {todo(filename)} ).collect::<Vec<f64>>(); 
//.collect creates a vector of mentioned type. by default a vector of output of function todo. 
//filenames is a collection of input which is iterable. eg. list of numbers, list of file paths, a vector.
//.par_iter() converts filenames into iterable.
//.map is error handling. if filename is None then it will make the cores stop. 
// |filename| todo(   filename.to_string()  ) is a closure. It takes all things defined before it in the scope. also takes filename as parameter and runs the function todo on it
// check defination of the function todo first. It takes &string and returns Vec<i64>. So make sure you put correct type in todo(__filename__) and collect::Vec<__f64__>()   
println!("{:?}",vec1);

}

fn todo(filename:&String)->f64{
   //
   6.5+1.0
}
