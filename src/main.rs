// --- --- --- --- --- --- --- --- --- --- 
// These are open source tools, 
// TSAHDU provides free legal assistance. 
// author: waajacu
// contact: savethebeesandseeds@gmail.com
// --- --- --- --- --- --- --- --- --- --- 
#[macro_use] extern crate rocket;

mod mathematics;
mod cryptography;
mod language;
mod transformer;
mod server;
mod utils;
mod laws;
mod files;
mod figures;
mod catalogue;

#[launch]
fn tsahdu() -> _ {
  // // Interpet Law
  // let aux = laws::LawBook {
  //   pais:String::from("colombia"),
  //   instrumento:String::from("constitucion"),
  // };
  // laws::interpret_law(&aux);

  rocket::build().attach(server::stage())
}