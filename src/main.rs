mod buffer;
mod draw;
pub use buffer::*;
pub use draw::*;
use std::io::stdout;

fn main()
{
    let mut stream = stdout();
    let s = draw_input_box(&mut stream);
    let input: String = buffer(&mut stream, s); 
    println!("{input}");
}
