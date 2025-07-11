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
the function returned.This is not desirable, as it takes from us the ability to do something while it runs.
When working with parallel code, this would take from us the ability to start a parallel task while the 
first runs (bcs we gave away control).

This is the moment where we could reach for 'threads'. But threads are a very specific concurrency 
primitive and we said that we are searching for an abstraction.
What we are searching for is something that represents ongoing work towards a result in the future.
Whenever, we say "something" in Rust, we almost always mean a trait. Let's start with an 
incomplete definition of the Future trait:
*/

trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output>;
}

fn main() {
    println!("Hello, world!");
}
/* Looking at it closely, we see the following:
- It is generic over the output.
- It provides a function called poll, which allows us to check on the state of the current 
computation.
- (Ignore Pin() and Context for now, you don't need them for high-level understanding.)

Every call to 'poll()' can result in one of these two cases:

1. The computation is done, poll will return Poll::Ready
2. The computation has not finished executing, it will return Poll::Pending

This allows us to externally check if a Future still has unfinished work, or is finally done and can give 
us the value. 
The most simple (but not efficient) way would be to just constantly poll futures in a loop.
There are optimizations possible, and this is what a good runtime does for you.
Note that calling poll again after case 1 happened may result in confusing behaviour.


Async

While the Future trait has existed in Rust for a while, it was inconvenient to build and describe them.
For this, Rust now has a special: async. 

Example:
*/
async fn read_file(path: &str) -> io::Result<String> {
    let mut file = File::open(path).await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    Ok(contents)
} 

/* Amazing little difference, all we did was label the function async and insert 2 special commands: .await.
The async function sets up a deferred computation. When this function is called, it will 
produce a Future<Output - io::Result<String>> instead of immediately returning 
a io::Result<String>. (Or more precisely, generate a type for you that implements Future<Output = io::Result<String>> ) 


What does .await do?

The .await postfix does exactly  what it says on the tin: the moments you use it, the code will wait until the requested action
(e.g, opening a file or reading all data in it) is finished. The .await? is not special- 
it's just the application of the '?' operator to produce its value.
The .await point act as a marker. Here, the code will wait for a Future to produce its value. How will a future finish?
The marker allows the component (usually called the "runtime") in charge of executing this piece of code to take 
care of all the other things it has to do while the computation finishes. It will come back to this point 
when the operation you are doing in the background is done..
When executing 2 or more of these functions at the same time,  our runtime system is then able to fill time with 
handling all the other events curently going on. 

*/

// Tasks - let us run Futures, in async-std, the task module is responsible for this. The simplest way is using the block_on function.

use async_std::{fs::File, io, prelude::*; task};

async fn read_file(path: &str) -> io::Result<String> {
    let mut file = File::open(path).await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
}

fn main() {
    let reader_task = task::spawn(async {
        let result = read_file("data.csv").await;
        match result {
            Ok(s) => println!("{}", s),
            Err(e) => println!("Error reading file: {:?}", e)
        }
    });
    println!("Started task");
    task::block_on(reader_task);
    println!("Stopped task");
}

// TASKS - in async-std, the task module is responsible for this. The simplest way 
// using the block_on function

use async_std::{fs::File, io, prelude::*, task};
async fn read_file(path: &str) -> io::Result<String> {
    let mut file = File::open(path).await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    Ok(contents)
}

fn main() {
    let reader_task = task::spawn(async {
        let result = read_file("data.csv").await;
        match reasult {
            Ok(s) => println!("{}", s), 
            Err(e) => println!("Error reading the file: {:?}", e)
        }
    });
    println!("Started trask:");
    task::block_on(reader_task);
    println!("Stopped task");
}

/*
The async block ...'let result = read_file("data.csv").await;....' 
Async blocks are neccessary to call async functions, and will instruct the compiler to include all the relevant instructions to do so. 
In Rust, all blocks return a value and async blocks happen to return a value of the kind Future.

..the interesting part : task::spawn(async {});

spawn takes a Future and starts running it on a Task. It returns a JoinHandle.Futures in Rust are sometimes called cold Futures. 
You need something that starts running them. To run a Future, there may be some additional bookkeeping required, e.g, whether it's running or finished, where it is being placed in memory
and what the current state is. This bookkeeping is part abstracted away in is a Task.

A Task is similar to a thread, with some minor differences: it will be scheduled
by the program  instead of the operating system kernel, and if it encounters a point where it needs to wait, a program iself is responsible for waking it up again.
An async_std task can also have a name and a ID, just like a thread.

For now, its enough to know that Once you have spawned a task, it will continue running in the background. The JoinHandle is itself responsible that will finish once the task has run to conclusion
Much like with Threads and the Join function, we can now call block_on on the handle to block the program (or the calling thread, be specific) and wait for it to finish. 


*/