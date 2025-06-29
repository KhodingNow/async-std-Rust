// Abstracting over computation - a type that 
// - DESCRIBES a computation without running it:

fn read_file(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
/*
Speaking in terms of time, we can only take the action before calling the function or after 
the function returned.This is not desirable, as it takes from us the ability to something while it runs.
When working with parallel code, this would take from us the ability to start a parallel task while the 
first runs (bcs we gave away control).

This is the moment where we could reach for 'threads'. But threads are a very specific concurrency 
primitive and we said that we are searching for an abstraction.
What we are searching for is something that represents ongoing work towards a result in the future.
Whenever, we say "something" in Rust, we almost always mean a trait. Let's start with an incomplete definition of the Future trait:
*/

trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output>;
}

fn main() {
    println!("Hello, world!");
}
