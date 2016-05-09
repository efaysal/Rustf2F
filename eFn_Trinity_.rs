
// Make it fully generic without any extrat-library to add, only using what Rust has.

// Simple functions to check the Rust compiler trinity doctrine for a self-defined function "fn".

fn check_fn<F:Fn(Input) -> Result< Output1,Output2> + Sync + Send,Input, Output1,Output2>(_: F) { println!(" It has Fn trait.  "); }
fn check_fnmut<F:FnMut(Input) -> Result< Output1,Output2> + Sync + Send,Input, Output1,Output2>(_: F) { println!(" It has FnMut trait.  "); }
fn check_fnonce<F:FnOnce(Input) -> Result< Output1,Output2> + Sync + Send,Input, Output1,Output2>(_: F) { println!(" It has FnOnce trait.  "); }


// Simple functions to check the nature of the closure.

fn check_executionfunction_fn<F,T: ExecutionfunctionFn<F>>(_: T) { println!(" It's a Fn closure.  "); }
fn check_executionfunction_fn_mut<F,T: ExecutionfunctionFnMut<F>>(_: T) { println!(" It's a FnMut closure.  "); }
fn check_executionfunction_fn_once<F,T: ExecutionfunctionFnOnce<F>>(_: T) { println!(" It's a FnOnce closure.  "); }

// Closure Fn!
 pub trait ExecutionfunctionFn<Input>: Send + Sync {
	  type Output1;
	  type Output2;
	 fn execute(&self, params: Input) -> Result<Self::Output1,Self::Output2>;
}
impl<F,Input, Output1,Output2> ExecutionfunctionFn<Input> for F where F: Fn(Input) -> Result< Output1,Output2>, F: Sync + Send {
  type Output1 =  Output1;
  type Output2 = Output2;
  fn execute(&self, params: Input) -> Result< Output1,Output2> {
      // To check its external enviroment capture.
   println!(" The compiler picks Fn, used the external enviroment by reference: &self ");
   println!(" ");
    self(params)
  }
  }

  // Closure FnMut!
   pub trait ExecutionfunctionFnMut<Input>: Send + Sync {
  	  type Output1;
  	  type Output2;
  	 fn execute(&mut self, params: Input) -> Result<Self::Output1,Self::Output2>;
  }
  impl<F,Input, Output1,Output2> ExecutionfunctionFnMut<Input> for F where F: FnMut(Input) -> Result< Output1,Output2>, F: Sync + Send {
    type Output1 =  Output1;
    type Output2 = Output2;
    fn execute(&mut self, params: Input) -> Result< Output1,Output2> {
        // To check its external enviroment capture.
        println!(" The compiler picks FnMut, used the external enviroment by a mutable reference: &mut self ");
        println!(" ");

      self(params)
    }
    }

    // Closure FnOnce!
     pub trait ExecutionfunctionFnOnce<Input>: Send + Sync {
        type Output1;
        type Output2;
       fn execute(self, params: Input) -> Result<Self::Output1,Self::Output2>;
    }
    impl<F,Input, Output1,Output2> ExecutionfunctionFnOnce<Input> for F where F: FnOnce(Input) -> Result< Output1,Output2>, F: Sync + Send {
      type Output1 =  Output1;
      type Output2 = Output2;
      fn execute(self, params: Input) -> Result< Output1,Output2> {
          // To check its external enviroment capture.
       println!(" The compiler picks FnOnce, copied its external enviroment: self ");
       println!(" ");
        self(params)
      }
      }

fn main()
{

//  From generic types to some actual provides types by Rust!

  // a simple self-defined function, using mainly important element in Rust design as Result-Option.
 fn plus2(i: f64) -> Result<bool,Option<i32>>
 {
     let y = i as i32;
     Ok( y == 1  )
 };

println!("");
println!("Check if  this given self-defined function  enjoys the trinity? ");
println!("");
check_fn(plus2);
check_fnmut(plus2);
check_fnonce(plus2);
println!("");

println!("Check its closure nature? ");
println!("");
check_executionfunction_fn(plus2);
check_executionfunction_fn_mut(plus2);
check_executionfunction_fn_once(plus2);

println!("");
println!("Discovering the magic of the Rust compiler to pick up one closure among many! ");
println!("All our closures, Fn*, have the function, execute.");
println!("When the self-defined function is used as an argument in execute function,");
println!("which one the compiler will certainly use?");
println!("");

println!("");
println!(" First Example ... ");
println!("");

let t=1.5;
match plus2.execute(t) {
Ok(v) => println!("Result is {}", v),
Err(e) => match e
{
Some(x) => println!("Error is {} ", x),
None => println!(" Nothing to process ... ")
			 },
}

println!("");
println!(" Second Example ... ");
println!("");

let t=4.1;
match plus2.execute(t) {
Ok(v) => println!("Result is {}", v),
Err(e) => match e
{
Some(x) => println!("Error is {} ", x),
None => println!(" Nothing to process ... ")
			 },
}

}
