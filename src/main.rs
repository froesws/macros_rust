use macros_example::GetFields;
use macros_example::GetFieldsValues;
use macros_example::HelloWorld;
use macros_example::PrintFields;

trait HelloWorld {
    fn hello_world();
}

trait GetFields {
    fn get_fields() -> Vec<&'static str>;
}
trait PrintFields {
    fn print_fields();
}

trait GetFieldsValues {
    fn get_fields_values(&self) -> Vec<String>;
}

#[derive(PrintFields, GetFields, GetFieldsValues)]
struct ToastedBread {
    a: i32,
    b: bool,
    c: String,
}

#[derive(HelloWorld)]
struct FrenchToast;

#[derive(HelloWorld)]
struct Waffles;

fn main() {
    FrenchToast::hello_world();
    Waffles::hello_world();
    let toasted = ToastedBread {
        a: 1,
        b: true,
        c: "Hello, World!".to_string(),
    };
    ToastedBread::print_fields();

    println!("Fields: {:?}", ToastedBread::get_fields());
    println!("Values: {:?}", toasted.get_fields_values());
}
