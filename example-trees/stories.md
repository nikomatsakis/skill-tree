# Harmonic synthesis stories

See also:

* https://hackmd.io/YkY-62ZRQ9uevOD6iF7e0w?view

# Characters

Pick the least experienced character that you feel can be successful without help:

* Niklaus: Minimal programming experience
    * Something that is 
* Alan: Significant general programming experience
    * Something you would only be able to do easily if you've been programming a while.
* Grace: Significant systems programming experience
    * Something that you would only be able to do easily if you understand low-level system details.
        * Doesn't mean you've been coding in C++
        * Building high performance systems in Java likely counts
* Barbara: Significant Rust programming experience
    * Something that you would only be able to do easily if you understand Rust pretty well.

# TODO

* Send bounds for traits, where do they fit in?
* Huge futures and how to manage them?

# Template

* Status quo stories
* Story
* Why X as the main character?
* What are they happy with?
* What are they disappointed with?

# Async traits are possible

* Status quo stories
* Story
    * Barbara needs to make a trait for some async crates she is working on
    * Service trait `trait Service { type }`
    * Async fn trait (http requests) ...
        * Maybe Barbara helps Alan?
    * Barbara needs to write a function that requires the futures be send
        * It is kind of painful
    * Third: For dyn futures
        * Send, not Send
* Why Barbara as the main protagonist?
    * To be successful, requires Rust fluency or at least a friend
* What is she happy with?
    * It works and is possible
    * impl Trait and generic associated types enable a bunch of different things
* What is she disappointed with?
    * She wishes she could write `async fn`
        * Addressed in async fn everywhere
    * Requiring that futures be send is pretty painful
    * Working with trait objects is a pain

# Async fn everywhere

* Status quo stories
* Story
    * Part 1. Niklaus writes some async functions and closures.
        * Niklaus can write `async fn` in traits and things work
        * He is converting some code from a blog post
            * It uses a closure
            * He manages to map it to an async closure and it works
        * He writes a recursive functions and it's fine
        * To get dyn trait working? Has to consult Barbara, but there's a cryptic line of code he can write
    * Part 2. Grace explores dyn Trait
        * Grace explors the various knobs one can turn around dyn Trait
        * To get dyn trait working? Has to consult Barbara
        * To deal with Send bounds? What happens here?
* Why Niklaus and Grace as the main protagonists?
    * Working with async fn and closures ought to be something one can learn without needing significant programming experience.
    * Why switch to Grace? To readily work through the options around dyn Trait requires understanding systems programming details.
* What is he happy with?
    * Mostly he is able to just write "async fn"
* What is he disappointed with?
    * Getting dyn trait working requires some semi-cryptic annotations with tradeoffs that he doesn't really understand
        * How can we extend the story to cover that part?

# Async iteration is awesome

* Status quo stories
* Story
* Why X as the main character?
* What are they happy with?
* What are they disappointed with?

# Async read and write are a pleasure to use

* Status quo stories
* Story
    * Niklaus mixes and matches various crates from crates.io to do something cool
    * Niklaus has to manage input/output over the same stream
    * Alan wraps a reader to track bytes (??)
    * 
* Why Niklaus and Alan as the main character?
    * Niklaus for kind of basic programming tasks and how nice they are
    * Alan for bringing specific patterns to bear (wrapping readers) 
* What are they happy with?
    * Things work
* What are they disappointed with?

# Portability is possible

* Status quo stories
* Story
    * Alan grabs a DNS library and SLOW protocol
        * He has to write a bit of glue code to connect it up
    * Alan writes a library that works across runtimes
        * He has to use the right traits in the right places
        * Uses `dyn Trait` to reduce generics
    * Alan loads another library
        * They use generic parameters everywhere
* Why Alan as the main character?
    * Q: Is this easy enough for Niklaus?
* What are they happy with?
    * It is possible
* What are they disappointed with?
    * Glue code is a bit annoying compared to node.js or what have you
    * Not everything they want is in the trait
    * Sometimes they encounter libraries that just bake in a runtime because you can't make the most ergonomic APIs you might want

# Getting started in async Rust is a smooth experience

* Status quo stories
* Story
    * Niklaus wants to learn async Rust
    * Niklaus comes to the async book, it recommends that a list of runtims
        * "Specialized for different purposes"
        * "Pick one for now, it's easy to change later"
    * Niklaus writes `async fn main` with their chosen runtime
    * Niklaus gets lints that guide him against blocking functions and give helpful suggestons
    * Niklaus gets a warning about a really large future and a suggestion to add `repr(box)`
        * He is referred to the book and reads about it
        * He resolves to dig a bit more later on
    * Niklaus gets referred to the book by a lint which explains some problem
        * The book suggests a design pattern
        * He tries it and feels he learned something
    * Niklaus experiments with other runtimes; they seem to work. He is happy.
* Why Niklaus as the main character?
    * Our goal is that people should be able to learn Async Rust with minimal programming experience and be productive.
* What are they happy with?
* What are they disappointed with?

# Resolving problems in running applications is easy

* Status quo stories
* Story
    * Alan has a simple logic bug
        * He opens gdb and rr, explores
        * He is able to set breakpoints, use next and reverse, and that sort of thing
    * Later, Alan has a blocked task
        * He opens up gdb but `info threads` is not useful
        * He pops open the book and learns about Turbowish
    * He goes to Turbowish and gets a list of his tasks
        * He sees a :warning: emoji and goes to investigate
        * Ah-ha, seems like the problem is X
    * He sees other flags popping up and fixes some other problems
* Why Alan as the main character?
    * Assuming experience with debuggers and resolving problems
* What are they happy with?
    * Tools that work pretty well
* What are they disappointed with?
    * A few too many distinct tools
    * Integration into gdb/rr might be even better

# Me make Rust FAST

* Status quo stories
* Story
    * A client complains that their requests are slow
        * Pop open turbowish, filter down to tasks from that client
        * Compare to the general flamegraph
        * Ah, this peak looks different
        * Investigate, find an O(n^2) loop (or something)
    * Warning about large futures that are being copied
    * Advice based on production profiling data
* Why Grace as the main character?
* What are they happy with?
* What are they disappointed with?

# Easily manage async cleanup

* Status quo stories
* Story
    * Alan is working with an SQLite database
        * Drop works as expected
    * Ability to highlight a specified "close" function that is *not* Drop, perhaps it takes parameters
    * Some kind of warning for sequential drop when async drop is available
        * Perhaps a case where it goes awry -- and what happens, anyway? -- when using a generic function!
* Why Alan as the main character?
    * Not sure! Maybe Niklaus
* What are they happy with?
    * Things mostly work
* What are they disappointed with?
    * They would prefer static enforcement

# Portability across Send

* Status quo stories
* Story
    * 
* Why Alan as the main character?
* What are they happy with?
* What are they disappointed with?

# If it compiles, it works

* Status quo stories
* Story
    * Alan is working on 
* Why Alan as the main character?
    * "If it compiles, it works" requires some level of experience to get the logic right.
* What are they happy with?
* What are they disappointed with?

# Zero copy

* Status quo stories
* Story
    * 
* Why Alan as the main character?
* What are they happy with?
* What are they disappointed with?

# Testing your async code is easy

* Status quo stories
* Story
    * 
* Why Alan as the main character?
* What are they happy with?
* What are they disappointed with?

# Long-running sequential loops are easy to find and remedy

* Status quo stories
* Story
    * 
* Why Alan as the main character?
* What are they happy with?
* What are they disappointed with?

