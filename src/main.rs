 # [macro_use ]extern crate rocket ;
#[get("/")]

 fn hi () -> & 'static str {
    "Welcom"
 }
  #[launch]
fn rocket ()-> _ {
 rocket ::build().mount( "/", routes![hi])
}