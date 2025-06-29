FUTURES:
A notable point about Rust is fearless concurrency. That is the notion that you  should be empowered to do concurrent things, without giving up safety. Also, Rust being a low-level language, it's about fearless concurrency without 'picking a specific implementation strategy. This means we must abstract over strategy, to allows choice later, if we want to have any way to share code between users of different strategies.
Furures abstract over computation. They describe "what", independent of the "where" and the "when". For that, they aim to break code into small, composable actions that can then be executed by a part of our system.

Send and Sync

Luckily, concurrent Rust already has two well-known and effective concepts abstracting over sharing between concurrent parts of a program: Send and Sync.
Notably, both the Send and Sync traits abstract over strategies of concurrent work, compose neatly, and don't prescribe an implementation.
As a quick summary;
- Send abstracts over passing data in a computation to another concurrent computation (let's call it the receiver), losing access to it on the sender side. In many programming languages, this strategy is commonly implemented, but missing support from the language side, and expects you to enforce the "losing access" behaviour yourself. This is a regular source of bugs: senders keeping handles to send things around and maybe even working with them after sending. Rust mitigates this problem by making this behaviour known. Types can be Send or not (by implementing the appropriate marker trait), allowing or dissallowing sending them around, and the ownership and borrowing rules prevent subsequent access.

- Sync is about sharing data btwn two concurrent parts of a program. This is another common pattern: as writing to a memory location or reading while another party is writing is inherently unsafe, this access needs to be moderated through synchronization. There are many common ways for two parties to agree on not using the same part in memory at the same time, for example 'mutexes' and 'spinlocks'. Again, Rust, gives you the option of (safely) not caring. Rust gives you the ability to express that something needs synchronisation while not being specific about the how.

Now that we have avoided any word like "thread", but instead opted for "computation". The full power of Send and Sync is that they relieve you of the burden of knowing WHAT shares. At the point of implementation, you only need to know which method of sharing is appropriate for the type at hand. This keeps reasoning local and is not influence by whatever implementation the user of that type later uses.

Send and Sync can be composed in interesting fashions. 

To sum up: Rust gives us the ability to safely abstract over important properties of the concurrent programs, their data sharing. It does so in  a very lightweight fashion; the language itself only knows about the two markers Send and Sync and helps us a little be deriving them itself, when possible. The rest is a library concern.

An EASY view of computation.
While computation is a subject to write a whole book about, a very simplified view suffices for now: A sequence of composable operations which can branch based on a decision, run to succession and yield a result or yield an error.

DEFERRING computation.

As mentioned above, Send and Sync are about data. But programs are not only about data, they also talk about computing the data. And that's what FUTURES do. 
WHAT do FUTURES allow us to express: - they go from this plan;

- Do X
- If X succeeded, do Y

towards:

- Start doing X
- Once X succeeds, start doing Y.

This is about deferring computation: Instead of telling the computer what to execute and decide upon nOW, you tell it what to START doing and how to react on potential events in the...well...Future.
