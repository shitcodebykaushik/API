#![allow(dead_code ,unused_variables)]
use std::future::Future;

 fn main (){
   println!("Hello world laila ");
   let x = foo1();
 }
 async fn foo1 ()-> usize {
   println!("hiuo");
   0
 }
  fn foo2 () -> impl Future<Output = usize >{
   async {
      println!("foooo");
      0
   }
  }
