# First-pass blog post drafts

These are rough drafts for every item in `blog-post-ideas.txt`.

They are written as post openings or early sections, but the work has not been
done yet. Any command output, benchmark result, compile result, source claim, or
runtime behavior that needs verification is marked with:

`TODO: actually run this and check`

## 1. GCC extension for expression statements

I wrote a C macro that evaluates each argument once, stores the results in
temporary variables, and still behaves like an expression. In ordinary C that
combination is awkward, because a block of statements is a block of statements,
and an expression is an expression. GNU C has an extension that blurs the line,
written with the weird `({ ... })` syntax.

TODO: actually run this and check: compile a tiny `MAX(x++, y++)` example with
GCC and Clang in GNU mode, then compile it with strict C flags and confirm which
diagnostics appear.

The interesting part is that this is not a toy feature invented for blog posts.
It explains a lot of real C macro style. The macro can introduce local
variables, use `__auto_type` or `typeof`, and return the value of the last
expression in the block. The cost is that the program is now GNU C, and the next
person trying to build it with a stricter compiler gets to discover that fact.

## 2. MicroPython and CircuitPython

I put the same small program on a board under MicroPython and CircuitPython,
because "Python on a microcontroller" is too broad a phrase to be useful. A
blinking LED is not interesting by itself, but the deployment loop, module
support, filesystem behavior, and memory errors are where the two projects start
to feel different.

TODO: actually run this and check: use the same board, the same LED program, and
one small sensor or serial example under both runtimes, then record install
steps, memory output, and library differences.

The post should probably be about what gets removed to make Python fit into a
small device. Ordinary Python programmers take imports, files, exceptions,
floating point, and package installation for granted, and all of those become
real engineering choices when the machine is tiny. The comparison is useful only
if it includes the annoying parts of the workflow, because that is where the
runtime stops being a logo and becomes a thing you have to use.

## 3. Large files make Git slow

I made a few Git repositories with different kinds of badness: many small files,
one large file, and one large file changed many times. Git is very good at
source code history, but a repository full of giant blobs makes it do work that
looks much less like version control and much more like object-store abuse.

TODO: actually run this and check: create the test repos, measure `clone`,
`checkout`, `status`, `gc`, and `repack`, then compare plain Git with Git LFS.

The first thing to inspect is the `.git/objects` directory before and after
packing. A large binary that changes slightly is a different problem from a
large binary that changes completely, and both are different again from a
directory full of small text files. This is the kind of post where the useful
answer is probably a table of timings and repository sizes, followed by the
simple conclusion that Git can store almost anything and that this is not the
same as it being happy.

## 4. `hyperfine` for profiling

I used `hyperfine` to time two tiny commands, then changed the benchmark setup
until the winner changed. That is the useful thing about a benchmarking tool:
it can make bad measurements look more obviously bad if you are willing to poke
at the setup.

TODO: actually run this and check: benchmark a few commands with warmups, cold
cache behavior, shell startup included and excluded, and a parameter sweep.

`hyperfine` gives you repeated runs, summary statistics, warmups, and exports,
which is already better than staring at one invocation of `time`. It still does
not know what you meant to measure. If the command reads a file, the page cache
matters. If the command is tiny, shell startup matters. If the result is
surprising, the next step is probably `perf stat` or a profiler rather than a
more decorative bar chart.

## 5. CWG 1331 and the `std::optional` mistake

I tried to write a small optional-like type and ran into a rule about defaulted
copy constructors that feels much too obscure for the surface area it affects.
The issue is CWG 1331, and the annoying part is that `std::optional` exposes a
language rule most users would never think to ask about.

TODO: actually run this and check: build the smallest example from the core
issue, compile it on several compiler versions, and compare the diagnostics with
the standard wording.

The post should start with the class definition rather than the committee issue.
If the example looks as though it should copy, and the compiler or standard says
otherwise, the reader has a reason to care. Only after that does it make sense
to talk about defaulted special members, const-compatible subobjects, and why a
library type as simple-looking as `optional` gets dragged into the machinery.

## 6. C++ `std::error_code` and `<system_error>`

I rewrote a tiny POSIX wrapper using `std::error_code`, mostly to see whether
the standard library abstraction earns its keep. The raw `errno` version is
ugly in a familiar way. The C++ version is more principled, but it takes several
namespaces and a category object to say what C says with one global integer and
a sigh.

TODO: actually run this and check: write wrappers around `open`, `read`, and a
made-up domain error, then compare `errno`, `std::error_code`,
`std::system_error`, and `std::expected`.

The interesting question is whether `std::error_code` is clunky because it is
bad, or because the underlying problem really is messy. A portable error value
needs a domain, a numeric code, a message, comparison semantics, and some way to
avoid pretending that every operating system has the same errors. That is a lot
of apparatus, but it is also the apparatus you need once `errno` is no longer
enough.

## 7. GitHub Dependabot action for Maven auto-PRs

I set up Dependabot on a small Maven project and let it open dependency update
PRs. The cheerful version of this story is that GitHub notices stale
dependencies and sends patches. The useful version includes the transitive
dependency that changes behavior, the CI failure that only happens under one
profile, and the question of whether merging patch updates automatically is
really boring enough to trust.

TODO: actually run this and check: create a Maven test project, configure
Dependabot, capture the generated PR, and add CI that runs enough tests to make
the update meaningful.

Maven makes this a good example because dependencies are already a graph rather
than a list. The post should inspect the generated diff, the effective POM, and
the dependency tree before and after the update. If the final advice is merely
"turn on Dependabot", the post has failed, because the real question is which
updates can be treated as routine and which ones need a human to read the tree.

## 8. `hardbool`

I tried a boolean-like integer type with stricter valid representations, because
C makes it very easy for a value that conceptually means true or false to carry
some other bit pattern. `hardbool` is interesting precisely because it is small:
it asks whether the compiler can help enforce the two-value story people were
already telling themselves.

TODO: actually run this and check: find a compiler build that supports the
attribute or patch, compile examples with invalid bit patterns, and check
diagnostics or runtime behavior.

This is an embedded-systems-shaped problem. If a byte comes from hardware,
shared memory, a wire format, or a corrupted store, then "I only assign true and
false" is not much of an argument. The post should show a value becoming invalid
through a path that ordinary code might miss, then show exactly what the
extension catches and what remains outside its reach.

## 9. Kafka is not a queue

I built the same tiny work-dispatch example with a normal queue and with Kafka.
The Kafka version can be made to work, but the shape is different enough that
calling it a queue hides more than it explains. Kafka stores an append-only log,
consumers keep offsets, and messages do not vanish simply because one worker
handled them.

TODO: actually run this and check: implement a small job processor with Kafka
consumer groups, retries, and failures, then compare behavior with a simple
database-backed queue or RabbitMQ example.

The weirdness appears when a worker fails halfway through a job, when a message
needs retrying, and when a second consumer wants to read the same history for a
different purpose. In a queue, deletion or acknowledgement is central. In Kafka,
retention and replay are central. Those are both useful designs, but they make
different promises, and the post should show the promises being tested rather
than recite them.

## 10. TLA+ or something similar in Haskell or OCaml

I modelled a small concurrent system in TLA+ and then tried to express the same
thing in a typed functional language. The point is not to crown a winner, but to
see which tool makes the bug easiest to state. A model checker is useful when
it finds the bad interleaving you were too lazy or too optimistic to imagine.

TODO: actually run this and check: pick a tiny protocol, write the TLA+ spec,
run the model checker, then implement an executable model in Haskell or OCaml.

The post should start with the bug. A lock service, retry loop, or two-phase
state transition is enough, as long as there is a reachable state that should
not be reachable. After that, the comparison becomes concrete: TLA+ gives you
states and temporal properties, while Haskell or OCaml gives you types and
normal programming tools. The useful question is where the mistake becomes
visible.

## 11. V8 doubles on the heap vs CPython integers on the heap

I wrote the smallest programs I could to allocate many numbers in JavaScript and
Python, then looked at what the runtimes actually store. The sentence "V8 puts
doubles on the heap" sounds weird until you remember that a dynamic language
value has to carry enough information for the runtime to know what it is.

TODO: actually run this and check: inspect V8 SMI and heap-number behavior with
available tracing flags, inspect CPython `PyLongObject` size, and measure memory
use for large numeric collections.

This should be a representation post rather than a language-war post. CPython's
integers are objects with headers, refcounts, and digit storage. V8 has tagged
values, small integers, heap numbers, and arrays that can change representation
depending on what you put in them. The surprise is not that either runtime is
stupid, but that "number" is not a storage layout.

## 12. x64 assembler in Python with PeachPy

I used PeachPy to generate an x86-64 function from Python. That sounds like a
joke until the generated code has to care about registers, calling conventions,
and instruction encodings in exactly the same way a normal assembler does.

TODO: actually run this and check: install PeachPy, generate a tiny function,
assemble or load it, call it from C or Python, and compare the output with
handwritten assembly.

The useful question is why an assembler embedded in Python would exist at all.
One answer is that Python is good at generating repetitive structured text, and
assembly can contain a lot of repetition once SIMD or ABI variants appear. The
post should show one example where generation removes real tedium, then ask
whether the extra layer made the program clearer or merely more elaborate.

## 13. Write C to generate an ELF binary

I wrote a C program whose output is another program. More specifically, it emits
enough ELF headers and machine code for Linux to execute the file. This is the
kind of thing that sounds like a compiler project until the first working
version turns out to be a few structs, a few offsets, and one very small piece
of code.

TODO: actually run this and check: generate the smallest x86-64 Linux ELF that
exits with a known status, then inspect it with `readelf`, `objdump`, and
`strace`.

The post should begin with `readelf` output from a normal executable, then strip
the idea down until only the loader contract remains. The kernel does not need
your sections to be beautiful. It needs enough information to map bytes into
memory and jump to an entry point. Once that is visible, an executable file
starts to feel much less mystical.

## 14. Locality of behaviour and spooky action at a distance

I took one small UI interaction and implemented it in a way where the behavior
is visible next to the markup, then in a way where the behavior is split across
handlers, state, and conventions. The htmx phrase is "locality of behaviour",
which is a good phrase because it points at a real annoyance: code where a
button's effect is nowhere near the button.

TODO: actually run this and check: implement the same tiny interaction in htmx
and in a state-heavy frontend style, then trace the files touched by one click.

This should not become a general rant about frontend frameworks. The useful
artifact is a trace: user clicks here, request goes there, state changes here,
DOM updates there. If the htmx version is clearer, the post can say so by
showing how little jumping around is needed. If the other version handles a
harder case better, that should be shown too.

## 15. Folly `CoreCachedSharedPtr`

I benchmarked many threads repeatedly copying the same `shared_ptr`, because
shared ownership sounds cheap until the refcount becomes a shared hot spot.
Folly's `CoreCachedSharedPtr` appears to exist for exactly this kind of
annoyance: avoid making every core fight over the same atomic counter.

TODO: actually run this and check: build a tiny Folly benchmark comparing
ordinary `std::shared_ptr`, `CoreCachedSharedPtr`, and a raw-pointer baseline
under increasing thread counts.

The post should make the refcount visible. A `shared_ptr` copy is not simply a
pointer copy, because ownership has to be counted, and that count is shared
mutable state. Once enough cores touch the same cache line, the abstraction
starts charging rent. The Folly design is interesting if it moves that cost
somewhere more tolerable, and suspicious if it only helps in a narrow pattern.

## 16. Find a network driver to port to OpenBSD

I picked a network device that OpenBSD does not support, or does not support in
the way I want, and started reading a driver for another operating system. This
is the right kind of kernel project because success is optional; the useful post
is the map of what has to be understood before packets move.

TODO: actually run this and check: choose the exact device, collect datasheets
and an existing Linux or FreeBSD driver, then identify the matching OpenBSD
driver APIs.

The first useful section is probably a tour of a working OpenBSD driver for a
similar card. Probe, attach, DMA setup, interrupts, descriptor rings, and
handoff to the network stack are all separate pieces, and each one has to match
OpenBSD's expectations rather than the source driver's expectations. If the port
does not work, the post can still be good if it explains exactly where the
translation became hard.

## 17. Undo and time-travel debugging

I used a reversible debugger on a small bug where the bad state appears after
the cause has already disappeared. Ordinary debugging makes you set a breakpoint
earlier and hope you guessed correctly. Time-travel debugging lets you inspect
the failure and then walk backwards to the point where the value changed.

TODO: actually run this and check: reproduce a small bug under `rr` or Undo,
record the session, reverse-continue to the bad write, and note which sources of
nondeterminism were captured.

The interesting question is what had to be recorded. A program is not only its
instructions; it sees syscalls, signals, scheduling decisions, time, memory
layout, and input from outside. The post should show one reversible session,
then explain the machinery only as far as needed to understand why replay works
and where it becomes expensive.

## 18. Haskell future monad vs Reactor Java

I wrote the same fanout operation in Haskell and in Java Reactor: call three
services, combine the results, handle one failure, and enforce a timeout. This
is a good comparison because the happy path is easy in every async framework,
and the unhappy path is where the abstraction starts to show its politics.

TODO: actually run this and check: implement the same example in Haskell and
Reactor, including cancellation, timeout, bounded concurrency, and one exception.

The post should include the code rather than describe the libraries in the
abstract. Reactor has a vocabulary of streams, monos, schedulers, and operators.
Haskell has types, effects, and whichever async abstraction is chosen. The
question is which version makes failure handling visible without turning the
program into plumbing.

## 19. Java 21 ADTs from XSD `choice`

I took an XSD with a `choice` and generated the ordinary Java representation.
The result is usually a class with several nullable fields or a shape that hides
the fact that exactly one alternative is valid. Modern Java has sealed classes,
records, and pattern matching, which means the schema's sum type can now look
like a sum type in Java too.

TODO: actually run this and check: find or write a small XSD with nested
`choice`, run a normal generator, then hand-write or generate a Java 21 sealed
representation and compile pattern matching over it.

The useful example is one where the old API permits an invalid state. If two
fields can be set at once, or none can be set, the Java type has failed to carry
the schema rule. A sealed interface with one record per branch is much closer to
the XML model, and the post should show how much validation moves from runtime
checking into the type shape.

## 20. Heap after GC as a missing Micrometer feature

I looked at a JVM heap graph and wanted the number after garbage collection,
because current heap usage is noisy in exactly the way dashboards like to be
noisy. The live set after GC is often the signal you care about when checking
whether a service is slowly retaining more memory.

TODO: actually run this and check: build a small JVM app with Micrometer, record
normal heap metrics, listen for GC notifications, and export heap-used-after-GC
as a custom gauge or timer-derived metric.

The post should show two graphs from the same program: ordinary heap used and
heap used immediately after collections. If the ordinary graph sawtooths while
the post-GC line climbs, the feature becomes obvious. The next question is how
Micrometer should expose it without every application copy-pasting the same GC
notification listener.

## 21. Pre-Java Oak

Before Java was Java, it was Oak, and it was aimed at embedded or consumer
devices rather than the later world of applets, application servers, and Android
phones. I found early Oak material and the first useful thing is how familiar it
already looks. The second useful thing is where it does not look familiar.

TODO: actually run this and check: find primary Oak manuals, early source, or
contemporary papers, quote short examples, and compare them with Java 1.0.

This should be language archaeology with evidence on the page. An early code
sample can show which parts of Java were already present: classes, C-like syntax,
managed memory, threads, or whatever the document actually contains. The point
is not to make Oak sound exotic. The point is to work out which parts of Java
were inherited from a smaller device-oriented project and which parts belong to
the later public story.

## 22. Windows ABI vs Itanium/System V struct splitting

I compiled small C++ functions that pass and return structs of different shapes,
then looked at where the fields actually go. The source code says "pass a
struct". The ABI has to decide whether that means registers, stack, hidden
pointers, splitting, classification, or some other rule that nobody wants to
remember until FFI breaks.

TODO: actually run this and check: compile examples on MSVC and on a System V
ABI target, inspect assembly, and compare the output with the ABI documents.

The examples should be deliberately boring: `{ int, int }`, `{ int, double }`,
small arrays, and a non-trivial C++ type. If the two platforms make different
choices, the post can show the assembly first and the ABI rule second. That is
much better than starting with ABI prose, because ABI prose is where attention
goes to die.

## 23. Futex

I wrote a toy mutex using the Linux `futex` syscall. The useful part of futexes
is that the uncontended path can stay in user space, while the kernel only gets
involved when a thread has to sleep or wake another thread. A mutex syscall on
every lock and unlock would be much too expensive for ordinary code.

TODO: actually run this and check: implement a minimal futex mutex, test it with
two threads, benchmark uncontended and contended cases, and compare with
`pthread_mutex`.

The post should make the race visible. You check a word, try an atomic
operation, and only ask the kernel to sleep if the value still says sleeping is
appropriate. The weirdness is that the kernel does not own the lock; it owns a
wait queue associated with an address. That separation is the whole trick, and
also the source of many careful edge cases.

## 24. Kotlin coroutines vs reactive for limiting concurrency

I wrote the same "fetch ten thousand URLs with at most fifty in flight" program
using Kotlin coroutines and using a reactive style. The happy path is not the
test. The test is what happens when one call times out, when cancellation
arrives, and when the limit is accidentally removed.

TODO: actually run this and check: implement both versions, add failure and
timeouts, measure peak in-flight requests, and inspect stack traces or logs.

This is really a post about where the concurrency limit lives. In a coroutine
version it might be a semaphore, dispatcher, or structured-concurrency boundary.
In a reactive version it might be an operator with a concurrency parameter. Both
can work, but the code should make the dangerous part easy to see, because the
bug is not a type error. The bug is waking up one morning and discovering that a
small service can issue ten thousand requests at once.

## 25. Kryo serialization: how does it work?

I serialized a small Java object graph with Kryo and looked at the bytes. Fast
serialization libraries often get described as if speed were a personality
trait, but the bytes have to contain some combination of class information,
field values, object identity, nulls, and references.

TODO: actually run this and check: serialize simple objects, registered classes,
unregistered classes, and cyclic graphs, then inspect byte output and benchmark
against Java serialization and Jackson.

The first useful surprise is probably how much registration changes the output.
If the reader can see class names vanish or IDs replace metadata, the
performance story becomes less handwavy. The second useful surprise is object
identity, because a serializer that handles graphs has to know whether two
references point to the same object or merely equal objects.

## 26. BetaArchive and Internet Explorer for Solaris

I found an installer for Internet Explorer on Solaris, which is a phrase that
still feels wrong after reading it several times. The post should be about
getting it to run, or getting as close as possible, rather than about laughing at
the existence of the thing.

TODO: actually run this and check: locate the installer, verify platform and
architecture requirements, install an appropriate Solaris environment, and
capture screenshots or failure logs.

The useful historical question is what Microsoft thought the Unix browser market
looked like. The installer, required libraries, desktop integration, and bundled
documentation are all evidence. If it runs, the screenshots matter. If it does
not run, the failure matters too, because dead software often teaches you by
complaining about the world it expected.

## 27. Apache Tika for metadata extraction

I fed a directory of mixed files into Apache Tika and compared the extracted
metadata with other tools. Tika is attractive because it promises one interface
over many formats, and that is exactly the kind of promise that should make you
ask what happens on bad input.

TODO: actually run this and check: test PDFs, Office files, images, audio files,
corrupt files, and password-protected files, then compare with `exiftool`,
`pdfinfo`, and manual inspection.

The post should include the wrong or surprising cases. A universal parser is
useful even when imperfect, but the imperfections are the contract you actually
live with. If Tika extracts text but misses metadata, or reports a title from
some stale internal field, the reader learns more than from another example
where everything works.

## 28. Graal and Truffle: write a BASIC interpreter

I wrote a very small BASIC interpreter and then moved it into Truffle. The point
is to see what Graal can optimize when the interpreter is structured in the way
Truffle expects. A language implementation framework is much easier to
understand when the language is small enough to fit in your head.

TODO: actually run this and check: implement `PRINT`, `LET`, `GOTO`, arithmetic,
and maybe `FOR`, then compare a plain interpreter with a Truffle-node version.

The post should start with a BASIC program, not with Graal architecture. Once
the program runs slowly in a boring interpreter, there is a reason to introduce
nodes, specialization, warmup, and partial evaluation. If the benchmark improves,
the next job is to explain which parts became visible to the optimizer and which
parts are still ordinary interpreter overhead.

## 29. Winning strategy in QE

I played or modelled QE, the board game where players bid money they effectively
create themselves. That rule is funny because the normal auction constraint is
gone, but the scoring penalty means the constraint has only moved. You can bid
anything; you cannot escape the consequences of having bid it.

TODO: actually run this and check: read the rules carefully, model a simplified
auction, simulate bidding policies, and compare the results with real game logs
or played examples if available.

The post should probably copy the Sidereal Confluence approach: turn the weird
rule into numbers. What is a company worth at different points in the game? How
bad is being the largest spender? When is overpaying rational because the other
players are also constrained by fear? The useful output is a few bidding rules
that are narrow enough to be wrong in interesting ways.

## 30. Do not use patching in Python

I wrote a Python test that passes because `mock.patch` replaces the awkward
thing, then changed the production code in a way that should matter and watched
the test keep passing. That is the real complaint about patching: it can test
the lookup path you invented for the test rather than the dependency boundary
the program actually has.

TODO: actually run this and check: build a small module with a bad dependency,
test it with `patch`, refactor it, and compare with an explicit fake or
dependency-injected version.

The post should not say patching is always wrong, because sometimes the world is
ugly and patching is the cheapest ugly tool. The better claim is narrower:
patching makes it easy to hide design problems, and a passing patched test can
give you less information than it appears to. A fake object passed explicitly
often says more about the program's shape.

## 31. JSON vs Arrow serialization/deserialization speed

I wrote the same dataset as JSON and as Arrow, then measured how long it takes
to get useful values back out. JSON is text with a universal handshake. Arrow is
typed columnar memory with a much stronger opinion about shape.

TODO: actually run this and check: create a dataset with numbers, strings,
timestamps, and nulls, then benchmark parse, read, write, filtering, and memory
use in Python and at least one compiled language.

The post should avoid declaring a winner too early. Arrow ought to do well when
the consumer wants columns of typed data and can avoid copying. JSON ought to
remain annoying and useful because everything can produce and inspect it. The
interesting result is likely the boundary: the smallest dataset where Arrow's
setup cost pays for itself, or the workflow where JSON's slowness is irrelevant.

## 32. Atomics on x86 vs SPARC

I compiled the same atomic operations for x86 and SPARC, then inspected the
instructions. `std::atomic` gives the source code a portable vocabulary, but
the hardware still has to implement ordering with whatever instructions and
memory model it actually has.

TODO: actually run this and check: compile relaxed, acquire, release, and
sequentially consistent examples for x86-64 and SPARC, then compare assembly and
barriers.

This fits nicely with old hardware work because SPARC makes some ideas visible
that x86 hides. On x86, strong ordering can make simple examples look almost too
easy. On weaker or different architectures, barriers and atomic instructions
show up in places that explain why the C++ memory model has so many names. The
post should put the assembly on the page before making claims about semantics.

## 33. Elixir and crashing as error handling

I wrote a tiny Elixir program with a worker that crashes at random, because the
phrase "let it crash" is too easy to repeat without checking what it means. The
worker does something boring, sometimes raises an exception, and sits under a
supervisor. In an ordinary script that would be the end of the program. In
Elixir, the supervisor notices and starts another copy.

TODO: actually run this and check: build the supervision tree, add a flaky
worker, capture logs, then add stateful work to see what survives a restart.

The interesting part is where the mess went. Restarting a worker is easy when
the worker has no important state and the failed operation can be retried. It is
much less comforting if the worker crashed halfway through writing a file,
sending an email, charging a card, or mutating state outside the process. The
post should find the boundary where "let it crash" is clean engineering and the
boundary where it is a slogan hiding a transaction problem.

## 34. Annotation processor for compile-time XSD

I wrote an annotation processor that reads an XSD during compilation and
generates Java types. The attraction is that schema mistakes and awkward XML
shapes should appear while compiling, rather than at runtime in a part of the
program that thought it was only parsing a message.

TODO: actually run this and check: implement a minimal annotation processor,
generate Java 21 records and sealed classes from a small XSD, and test
incremental compilation in an IDE or build tool.

The post should show the generated code. Annotation processing can sound like
dark magic until the generated file appears under `target` or `build`, at which
point it becomes a Java program writing another Java program. The awkward parts
are also worth showing: error reporting, source locations, build caching, and
how bad the developer experience gets when generation fails.

## 35. Transcript search with Rust and embedded SQLite

I rebuilt a small transcript search tool using Rust and SQLite FTS. The appeal
is deployment: one binary, one database file, and no separate PostgreSQL server
to keep alive for a personal search tool.

TODO: actually run this and check: import a transcript corpus into SQLite FTS5,
write a Rust CLI or web wrapper, measure indexing time, query latency, and file
size.

The post should compare this with the older transcript-search setup rather than
pretend the idea arrived from nowhere. PostgreSQL is powerful, but a personal
tool has different constraints from a shared service. If SQLite gives good
enough search with a simpler operational story, that is the result. If ranking,
tokenization, or update behavior becomes annoying, that is the result too.

## 36. Shell scripts as an escape hatch out of unit tests

I took a small shell script that started as glue and made it important enough to
break a build. This is the normal lifecycle of shell in a codebase: harmless,
useful, load-bearing, then somehow outside the test strategy.

TODO: actually run this and check: create or find a real build/deploy shell
script, add `shellcheck` and a small `bats` test, then identify which behavior
still escapes testing.

The post should be fair to shell. Shell is excellent for connecting programs and
awful when it becomes an untyped mini-application with quoting rules from a
folk-horror story. The useful question is which parts should remain shell and
which parts should move into a language with normal tests, data structures, and
error handling.

## 37. Compression algorithm implementation

I implemented a small compressor and decompressor for a toy format. The goal is
to make the vocabulary less mysterious: literals, matches, dictionaries,
prefix codes, headers, and the annoying requirement that decompression must
reverse exactly what compression wrote.

TODO: actually run this and check: choose Huffman, LZ77, or LZSS, implement both
directions, and test text, repeated data, binary files, and random bytes.

The post should begin with a string where the redundancy is obvious. Once the
toy compressor makes that string smaller, it should be fed data where it fails.
Random data is a good teacher because it punishes vague claims about
compression. A useful compressor is a bet about the shape of the input, and the
post should make that bet visible.

## 38. Performant rounding

I wrote a few rounding functions and tested them on negative numbers, halfway
cases, large values, and NaNs. This is necessary because the classic trick of
adding `0.5` and truncating is exactly the kind of code that seems fine until it
meets the rest of arithmetic.

TODO: actually run this and check: compare standard library rounding functions,
custom code, and CPU instructions, then inspect assembly and benchmark batch
rounding.

The post should decide which rounding problem it is solving. Rounding floats to
integers, rounding decimal money, and rounding large vectors of values are
different jobs. Once the job is fixed, the examples can be concrete: show the
wrong result, show the correct result, then ask how much the correct result
costs.

## 39. Rust `bumpalo`

I built a small tree structure with ordinary allocation and then with
`bumpalo`. The arena version is faster in the case where many objects share the
same lifetime, because allocation becomes pointer bumping and deallocation
becomes dropping the whole arena.

TODO: actually run this and check: implement a parser or tree builder, measure
allocation count, runtime, peak memory, and the lifetime errors that appear when
arena-owned values try to escape.

The Rust part is the interesting part. In C, arena allocation is often a
discipline you promise to follow. In Rust, the lifetimes make the arena's
ownership story visible in the types. That can be annoying, but it is the good
kind of annoying: the compiler is forcing the same lifetime assumption that made
the allocator fast in the first place.

## 40. Rust audio synthesizer latency

I wrote the smallest Rust synthesizer I could: one oscillator, one audio
callback, and a way to change a parameter while it runs. Producing a sine wave
is easy. Producing it on time, repeatedly, without blocking the audio thread is
where the real-time part starts.

TODO: actually run this and check: build the synth with `cpal` or another audio
crate, measure buffer sizes and latency, then deliberately add allocation,
locking, or logging to see what causes glitches.

The post should include one failure. A click, underrun, or delayed control
change is more informative than a perfect sine wave. Audio programming makes
ordinary application habits look bad: allocate later, lock casually, print a log
line, wait for a channel, and suddenly the user hears the mistake.

## 41. Write a gRPC service

I wrote a tiny gRPC service with one unary method and one streaming method, then
looked at the generated code. The interesting thing about gRPC is not that it
can call a function over the network. JSON APIs can do that too. The interesting
thing is the package of commitments: protobuf schemas, generated clients,
HTTP/2, deadlines, status codes, and streaming semantics.

TODO: actually run this and check: implement the service, call it from a client,
capture generated code, inspect HTTP/2 behavior if practical, and add deadline
and cancellation examples.

The post should show the `.proto` file early, because that is where the API
actually lives. Once the schema is on the page, generated code becomes less
mysterious. The annoying parts are worth including too: versioning a message,
representing errors, debugging a failed call, and deciding whether the extra
machinery is justified for the service being built.

## 42. How Java objects are laid out in memory

I made a Java class with one `int` field and used JOL to see how much memory it
actually used. The answer is not four bytes, because a Java object is not only
its fields. There is an object header, alignment, compressed references
depending on JVM settings, and sometimes padding that exists only because
machines prefer certain shapes.

TODO: actually run this and check: run JOL on small classes, arrays, strings,
inheritance examples, and compressed-oops settings, then record exact layouts.

The useful thing about JOL is that it turns folklore into output. You can see
the mark word, class pointer, field, and padding rather than waving vaguely at
JVM overhead. This matters when a program allocates millions of small objects,
because the overhead can dominate the data you thought you were storing.

## 43. GPTs to make a biography of your parents

I recorded a family-history interview, transcribed it, and tried to turn it into
a structured biography draft with an LLM. This is a dangerous use case because
the output can sound emotionally plausible while inventing details, smoothing
over uncertainty, or making someone's life sound like a LinkedIn profile.

TODO: actually run this and check: design interview questions, record or use a
consented sample, transcribe it, extract chronology, generate a draft, and mark
every claim that needs human verification.

The post should be about workflow and boundaries rather than sentiment. The
model can organize, summarize, and suggest questions. It should not invent dates
or motives. The valuable artifact may be less like a finished biography and
more like a well-indexed set of stories that a family member can edit without
starting from a blank page.

## 44. Bubblewrap containers

I used `bubblewrap` to run a shell with a different view of the filesystem. This
is a nice container post because it does not start with Docker. It starts with
namespaces, bind mounts, and the weird fact that a process can be made to see a
world assembled specifically for it.

TODO: actually run this and check: create a minimal `bwrap` command, add
read-only binds, a writable tmpfs, network isolation if available, and compare
with the equivalent Docker invocation.

The post should build the sandbox one option at a time. First make `/` strange,
then add `/bin`, then add enough libraries to run something, then remove network
or write access. The point is to make containers feel like kernel features being
composed, because that is more useful than treating them as a product category.

## 45. Polonius borrow checker

I found a small Rust program that the current borrow checker rejects and that
Polonius is meant to understand more precisely. That is the right way into the
topic, because otherwise borrow-checker discussions become abstract very
quickly.

TODO: actually run this and check: find canonical Polonius examples, test them
on stable and nightly Rust, enable any available Polonius flags, and record
which examples compile.

The post should keep the example on the page while explaining loans and
lifetimes. The useful claim is not that the old borrow checker is bad. The
useful claim is that a particular analysis loses track of a fact a human can
see, and that Polonius represents the fact differently. If the example still
fails today, that result should be stated plainly.

## 46. `Libfn/functional`

I rewrote a small C++ error-handling pipeline using `Libfn/functional`, assuming
this is the intended library from the note. The ordinary version has nested
checks around `optional` or `expected`. The functional version tries to make the
happy path read as a sequence of transformations.

TODO: actually run this and check: identify the exact repository, build it,
rewrite a small pipeline, and compare it with plain C++23 facilities.

This is a good place to be skeptical. C++ can imitate functional style, but the
result is sometimes clearer and sometimes a stack of template names doing
Haskell karaoke. The post should show both versions side by side, then judge the
library by whether it removes real control-flow noise rather than by whether it
feels clever.

## 47. `uv` being much faster than `pip`

I installed the same Python project with `pip` and with `uv`, then separated the
time into resolving, downloading, building, and installing. The headline claim
is that `uv` is much faster. The useful question is where the time disappears.

TODO: actually run this and check: benchmark fresh and cached installs for
small, medium, and ugly dependency graphs, then compare lockfile behavior and
resolver output.

This should be a packaging post rather than a Rust victory lap. A fast resolver
is useful because Python packaging has trained people to accept waiting as
normal. If `uv` changes the feedback loop, that matters. The post should also
include a case where network or wheel building dominates, because a fast tool
cannot outrun the wrong bottleneck.

## 48. Rust destructors are not guaranteed

I wrote a Rust type with a `Drop` implementation that prints a message, then
made the message disappear without using `unsafe`. The simplest way is
`std::mem::forget`, and it is a useful reminder that leaking a value is memory
safe even when it is bad resource management.

TODO: actually run this and check: run examples for `mem::forget`, `Rc` cycles,
panic abort, process exit, and ordinary drop order.

This feels odd if the mental model is "Rust has RAII like C++". Rust has
deterministic destruction in ordinary ownership-based control flow, which is
already very useful. It does not promise that every destructor runs by
providence. If unsafe code relies on `Drop` being unavoidable, the unsafe code
has assumed something Rust never promised.

## 49. Rust static reflection drama vs C++ reflection

I tried to solve the same boring reflection task in Rust and in C++: get field
names and field types at compile time. This is the right task because nobody
needs philosophy to understand why serializers, inspectors, and UI generators
want this information.

TODO: actually run this and check: identify the specific Rust static reflection
discussion, implement the example with Rust derives or proc macros, then compare
with current C++ reflection proposals or experimental compilers.

The post should keep the drama secondary. The interesting part is that both
languages want compile-time power without importing all the old metaprogramming
pain. Rust has proc macros, which are powerful but live outside normal type
checking in awkward ways. C++ has templates, proposals, and a long institutional
memory of cleverness becoming unreadable.

## 50. Codon as faster Python with a JIT/compiler

I ran a few Python programs through Codon: a numeric loop, some object-heavy
code, and a program that imports common libraries. The first benchmark is the
one Codon should win. The later examples are where "Python-like" and "Python"
need to be separated carefully.

TODO: actually run this and check: install Codon, benchmark representative
programs against CPython and PyPy if useful, and record unsupported features or
semantic differences.

The post should be generous about the thing Codon is trying to do. A compiled
Python-like language can be very useful even if it is not a drop-in replacement
for every Python program. The useful boundary is practical: what code can I take
from an ordinary Python project, what has to be rewritten, and which speedups
survive once the example stops being a toy loop?

## 51. Hazard pointers

I wrote a tiny lock-free stack and then had to face the awkward question: when
is it safe to free a node? Popping a node with atomics is one problem. Knowing
that no other thread still has a raw pointer to the old node is another problem,
and it is often the problem people leave until too late.

TODO: actually run this and check: implement or adapt a lock-free stack, show
the unsafe reclamation bug, then add hazard pointers and compare with epoch
reclamation.

Hazard pointers make readers publish which nodes they might touch. A writer that
wants to free a removed node first checks those published hazards. The post
should show this with a real timeline of two threads, because the mechanism is
much easier to understand as an interleaving than as a definition.

## 52. Varargs C++ functions cannot be coroutines

I tried to put `co_await` inside a C++ function with C-style varargs. The
compiler rejects it, and that rejection is a useful doorway into how coroutine
frames are built. A coroutine is transformed into a state machine with saved
parameters and locals. C varargs are a poor fit for that transformation.

TODO: actually run this and check: compile a variadic coroutine example, collect
diagnostics from GCC and Clang, and compare with a variadic-template coroutine
that does compile.

The post should connect this to the decompiled coroutine work. Once you have
seen a coroutine frame as a real allocation with slots in it, restrictions like
this look less arbitrary. The compiler has to know what to preserve across
suspension. A typed parameter pack gives it a very different job from an
old-style ellipsis and a `va_list`.

## 53. Rewinding C++ coroutines with memory trickery

I took the minimal coroutine example, inspected the frame layout for one
compiler build, and tried to change the saved state by writing into the frame.
This is implementation-dependent and undefined behavior, which is exactly why it
belongs in a post about how the compiler happened to lower one program.

TODO: actually run this and check: compile the coroutine with fixed compiler
flags, identify the state field, mutate it before `resume`, then repeat under
different optimization levels and compilers.

The useful result is not a technique anyone should use. The useful result is a
more physical understanding of what "resuming a coroutine" means in one
implementation. If changing a byte jumps to a different suspension point, that
is weird and educational. If it crashes immediately under another build, that is
also educational, and probably the healthier outcome.

## 54. Python ten million threads: no-GIL vs async

I tried to make Python handle a silly number of concurrent waiting tasks using
threads and using async. The no-GIL work makes this worth revisiting, but it
does not remove stack memory, scheduler overhead, OS thread limits, or the
difference between CPU parallelism and I/O concurrency.

TODO: actually run this and check: use a current free-threaded Python build,
compare many sleeping threads with many asyncio tasks, and record memory,
startup time, scheduling behavior, and throughput.

The post should choose a workload that is boring enough to expose overhead. If
each task sleeps and wakes, the useful numbers are how many can exist, how much
memory they use, and how long it takes to poke them all. If the workload becomes
CPU-heavy, the no-GIL story changes. That distinction should be visible in the
measurements.

## 55. `sysret_ss_attrs` AMD64 design mistake

I read the kernel workaround for an AMD64 `SYSRET` issue and had to remember
that 64-bit mode did not completely delete segmentation from history. The exact
bug lives in the details of returning from syscalls and restoring segment
selector state, which is already a sign that the post needs diagrams or code.

TODO: actually run this and check: identify the exact Linux or BSD workaround,
read the AMD documentation or erratum, and trace the syscall return path.

The post should start from the workaround rather than from CPU architecture in
general. A line of kernel code with a comment about `SYSRET` is a better hook
than an essay about segmentation. From there, explain what state `SYSCALL` and
`SYSRET` touch, what they leave alone, and why operating systems still carry
workarounds for decisions made deep in the ISA.

## 56. Linux `readv` and `writev`

I wrote a tiny program that sends a header and a body using `write`, then
rewrote it using `writev`. The first version allocates or copies a temporary
buffer merely to make the data contiguous. The second version gives the kernel a
small array of buffers and asks it to write them as one operation.

TODO: actually run this and check: implement file and socket examples, test
partial writes, benchmark against manual concatenation, and inspect syscall
counts with `strace`.

The weird part is that `writev` is both simple and easy to misuse. The buffers
must remain valid until the syscall has consumed them, and partial writes still
exist. The post should show the correct loop, because the one-shot example is
too clean and teaches the wrong habit.

## 57. Tearing of 64-bit Java types

I wrote a small Java program that tries to catch a torn `long` read. The Java
memory model has historical wording around non-volatile 64-bit values that is
surprising if you have only used modern 64-bit JVMs. The spec and the machines
people actually run are not always telling the same story.

TODO: actually run this and check: read current JLS wording, test on available
JVMs, look for old 32-bit JVM behavior, and compare plain, `volatile`, and
atomic long access.

The post should be careful here. If the test never catches tearing, that is not
proof the spec never allowed it. It only says this JVM on this machine did the
reasonable thing. The interesting part is why the language spec had to account
for machines where a 64-bit value might be moved as two 32-bit halves.

## 58. Is Tokio WASM-compatible?

I compiled a tiny Tokio program to WebAssembly and watched which assumptions
survived. "WASM" is too vague here, because browser WASM, WASI, and other
runtimes expose different worlds. Tokio can drive futures, but many Tokio
features assume timers, sockets, threads, or OS integration.

TODO: actually run this and check: compile small Tokio examples for browser WASM
and WASI, test timers, spawning, TCP, files, and feature flags.

The post should become a compatibility table only after the examples fail or
work. A future that is ready immediately is boring. A timer, TCP connection, or
spawned blocking task forces the runtime assumptions into view. The useful
answer is probably a list of which Tokio concepts are portable and which belong
to an operating system-shaped environment.

## 59. Is pip's dependency solver Turing-complete?

I tried to encode a small hard problem as Python package dependencies. The
phrase "Turing-complete" is probably the wrong claim, or at least a claim that
needs more care than it usually gets, but dependency resolution is absolutely a
place where innocent-looking constraints can create ugly search.

TODO: actually run this and check: read pip resolver behavior, encode SAT-like
constraints with package versions, and measure resolver behavior on increasing
instances.

The post should be precise enough to ruin the joke if the joke is false.
Version solving can be finite and still unpleasant. It can encode combinatorial
choice without being an arbitrary computer. The right result might be "this is
NP-hard-shaped in practice" rather than "Turing-complete", and that would be a
better post because it would say what is actually happening.

## 60. `rkyv` for zero-copy deserialization

I archived a Rust struct with `rkyv` and then read it back without building a
fresh owned object graph. That is the attraction of zero-copy deserialization:
the bytes are already in a layout the program can use.

TODO: actually run this and check: serialize a few structs with `rkyv`, inspect
the bytes, benchmark against `serde_json`, `bincode`, and postcard, and test
validation, alignment, and endianness assumptions.

The post should make the constraints visible. If bytes on disk are going to be
treated as structured data, layout matters. Pointers cannot be ordinary process
pointers. Alignment matters. Versioning matters. Validation matters. The
performance win is only honest if the post also shows the rules that make the
bytes safe to borrow.

## 61. Rust profiling: `hyperfine` and Codespeed

I timed a Rust CLI locally with `hyperfine`, then sketched what it would take to
catch performance regressions over time. Local benchmarking answers one
question: did this command get faster on this machine under these conditions?
A project usually wants a slightly different question answered after every
commit.

TODO: actually run this and check: benchmark a small Rust CLI with `hyperfine`,
add a microbenchmark where appropriate, and configure or mock a Codespeed-style
tracking setup.

The post should show where quick timing stops being enough. If a change saves
twenty percent locally, that is useful. If a future commit loses the twenty
percent and nobody notices, it was a diary entry rather than a guardrail. The
hard part is noise: CI machines, caches, CPU frequency, and workloads that are
too small to measure honestly.

## 62. Python `-O` optimization/debug flag

I ran a tiny script with `python -O`, mostly to check what the flag actually
does. The name makes it sound as though the interpreter is about to try harder,
but the visible behavior is stranger than that: assertions disappear and
`__debug__` changes.

TODO: actually run this and check: run examples for `assert`, `__debug__`,
docstrings under `-OO`, bytecode differences, and a few tiny benchmarks.

This matters because people sometimes use `assert` as if it were production
validation. It is not validation if a command-line flag can remove it. The post
should show the same function accepting bad input under `-O`, then measure
whether any meaningful speedup appears. If the optimization flag is mainly a
debug-behavior flag, it deserves to be described that way.

## 63. ARM Jazelle: JVM bytecode on the CPU

I read about ARM Jazelle and tried to find what it actually executed in
hardware. "A CPU that runs Java bytecode" sounds absurd now, but it makes more
sense in a world of Java ME phones and small devices where bytecode execution
was a platform concern.

TODO: actually run this and check: find public ARM Jazelle manuals, old device
references, emulator support if any, and examples of what trapped versus what
executed directly.

The post should be historical but still technical. Which bytecodes were handled?
What mode did the CPU enter? What did the operating system or JVM have to set
up? If the public documentation is incomplete, that should be part of the story,
because semi-documented hardware features have their own distinct smell.

## 64. `sockaddr` fake flexible array

I followed a `bind` call from C source into the socket headers and found the old
trick: pass a pointer to `struct sockaddr`, pass a length, and rely on the real
object being some larger address-specific struct. This is the kind of C API that
works because everyone knows the lie being told.

TODO: actually run this and check: inspect `sockaddr`, `sockaddr_in`,
`sockaddr_in6`, and `sockaddr_storage` on a few systems, then compile examples
that print sizes and offsets.

The post should show the structs. Once the fields and sizes are visible, the
cast from `sockaddr_in *` to `sockaddr *` stops looking like normal type use and
starts looking like an ABI convention with a costume on. The length argument is
doing much of the real work, and `sockaddr_storage` is the later attempt to make
the old pattern less cramped.

## 65. SJLJ exceptions and C++ exception performance

I compiled the same C++ exception example using different exception mechanisms,
or as close as the toolchain allows. Claims about exception performance are
often sloppy because they mix up the normal path, the throw path, code size,
unwind tables, and old setjmp/longjmp-based implementations.

TODO: actually run this and check: find a target/toolchain that supports SJLJ
exceptions, compile examples, benchmark throw and non-throw paths, and inspect
assembly or unwind data.

The post should start with what "zero-cost exceptions" actually means. It does
not mean throwing is free. It means the ordinary path tries not to pay much
runtime cost for the possibility of an exception. SJLJ moves costs around in a
different way, and the interesting result is where the bill appears.

## 66. gRPC request hedging benchmarks

I made a fake service where most requests return quickly and a few take much
longer, then put a hedging client in front of it. The client sends the first
request as normal, waits for a fixed delay, and if nothing has come back yet,
sends a duplicate. Whichever response arrives first wins, and the other request
is cancelled if cancellation gets there in time.

TODO: actually run this and check: implement the fake service, configure latency
distributions, test hedge delays, record p50, p95, p99, extra requests, wasted
work, cancellation timing, and server load.

In the low-load case, this should help when the slow request is slow for
accidental reasons. Under higher load, the duplicate requests may still improve
some individual calls while increasing total work. The post should be about that
exchange rate. Tail latency is bought with spare capacity, and the benchmark
should show how expensive the purchase becomes.

## 67. Conservative GC: do not keep track of references

I wrote a small C program under a conservative garbage collector and tried to
make an integer-looking value keep an allocation alive. Conservative GC is weird
because it collects memory without exact knowledge of every pointer. It scans
memory and treats plausible pointer values as roots.

TODO: actually run this and check: use Boehm GC, create examples with real
pointers and pointer-looking integers, and inspect which allocations are kept.

The tradeoff is visible in the example. A precise moving collector needs exact
maps and control over references. A conservative collector can work with C
programs that were not designed for precise tracing, but false positives can
keep memory alive and moving objects becomes much harder. The post should show
the compromise as behavior, not as a definition.

## 68. `std::inplace_vector` and L1 cache

I replaced a small `std::vector` with an inline-capacity vector and measured a
tight loop. The idea is simple: if the elements fit inside the object, there is
no heap allocation for the small case, and the data may be closer to the code
that uses it.

TODO: actually run this and check: use an available `inplace_vector`
implementation or standard-library preview, benchmark small sizes, inspect
object size, allocation count, and cache behavior.

The post should include the downside. Inline storage makes the containing object
larger, and fixed capacity is a real design choice. For a vector that usually
holds three items, this can be excellent. For a vector copied everywhere or
rarely used, it may be a large object pretending to be an optimization.

## 69. Double-wide atomic instructions: are they good yet?

I checked whether a 128-bit atomic compare-and-swap is actually lock-free on the
machine in front of me. This matters because many lock-free designs want to
update a pointer and a counter together, often to avoid ABA problems.

TODO: actually run this and check: test `std::atomic<__int128>` or platform
intrinsics on x86-64 and ARM64, inspect generated code, and check
`is_lock_free`.

The post should separate language support, compiler support, and hardware
support. A type can compile and still call into a lock. An instruction can exist
and still require alignment or feature flags. The useful answer is not whether
double-wide atomics exist, but whether they are boring enough to build on in the
specific environment being measured.

## 70. AVX-512 Galois-field instructions

I wrote a tiny GF(2^8) operation using AVX-512 GFNI instructions, because these
instructions sound as though someone let a finite-field enthusiast design part
of the CPU. The question is what they actually do to bytes.

TODO: actually run this and check: find hardware or an emulator with GFNI
support, implement a small affine transform or multiplication example, and
compare scalar, table-based, and vector versions.

The post should keep the maths concrete. A byte can represent an element of a
finite field, and certain transformations over those bytes show up in crypto,
checksums, and erasure coding. Once the scalar operation is on the page, the
instruction becomes less weird. It is still specialized, but specialized in a
way that real software can use.

## 71. McKinsey space travel

I collected a few startups that have the same smell: former consulting or elite
MBA operators applying polished company-building language to technically hard
frontier markets. Space is the funniest version because rockets and orbital
infrastructure are much less forgiving than a slide deck.

TODO: actually run this and check: pick concrete companies, verify founder
backgrounds, read websites, decks, job ads, technical claims, and customer
evidence.

This post needs evidence or it becomes empty sneering. Some of these companies
may be real and impressive. Some may be excellent at fundraising and vague about
the hard part. The useful question is what the operator-consultant pattern is
good at, where it fails, and whether the public materials reveal the difference
before reality does.

## 72. Review of IM-1776 issue on health

I read IM-1776 Issue VI, "Health of a Nation", as a magazine issue rather than
as a bundle of takes. The obvious danger is reviewing the politics you expected
to find instead of the essays that are actually on the page.

TODO: actually run this and check: get the full issue, read every essay, extract
the main factual claims, and check which claims are testable.

The review should probably be organized around claim quality. Some essays may
make concrete claims about medicine, food, institutions, exercise, or family
life. Some may be mostly mood. Mood is allowed in a magazine, but it should not
borrow the authority of evidence without paying for it. The post should say
where the issue is sharp, where it handwaves, and where the aesthetics are doing
too much work.

## 73. `nanobind` for Python bindings

I bound the same small C++ function with pybind11 and `nanobind`, then compared
the extension modules. Binding libraries are easy to praise in the abstract and
much easier to judge when you have to write the build file, expose a class, and
handle object lifetimes.

TODO: actually run this and check: build equivalent pybind11 and nanobind
modules, compare source size, build setup, binary size, import time, call
overhead, NumPy support, and lifetime behavior.

The post should start with one C++ API that is slightly annoying, not with a
hello-world function. Ownership, references, exceptions, and arrays are where
binding libraries earn their keep. If `nanobind` produces smaller binaries or
cleaner code, show the diff. If the advantage is marginal for small examples,
say that too.

## 74. Python performance tips, loops, calls, CPython vs Codon

I collected a few common Python performance tips and turned them into
benchmarks. Advice like "avoid function calls" or "never write Python loops" is
sometimes directionally useful and often useless without the size of the data,
the interpreter, and the work inside the loop.

TODO: actually run this and check: benchmark function-call overhead, loop
styles, builtins, list comprehensions, vectorized libraries, CPython, PyPy, and
Codon where appropriate.

The post should make each slogan earn its place. A function call matters in a
tight loop that does almost nothing. It matters much less when the function does
I/O or calls into C for a large array operation. Codon adds another useful
complication, because making Python-like code fast may involve leaving ordinary
Python semantics behind.

## 75. Python GIL can release in the middle of `+=`

I wrote a class whose `__iadd__` method does something visible, then used it to
break the comforting story that `x += y` is one indivisible operation. The bytecode
and the method calls matter. The GIL does not turn arbitrary Python statements
into transactions.

TODO: actually run this and check: inspect bytecode for `+=`, write custom
`__iadd__` and `__add__` examples, and investigate where CPython can release the
GIL through allocation or C extension calls.

The post should be careful about the exact claim. Some operations are atomic in
particular CPython implementations under particular conditions. That is not a
programming model. If an operation can call user code, allocate, or invoke C
that releases the GIL, the simple mental model is already gone.

## 76. Java JIT deoptimization when a feature flag flips

I wrote a Java loop with a branch controlled by a feature flag and watched what
the JIT did after the branch stayed stable for a long time. Speculative
optimization is reasonable: if the program has behaved one way for millions of
iterations, the JVM can compile for that world and keep a path back if the world
changes.

TODO: actually run this and check: build the benchmark, enable JIT logging, use
JITWatch if useful, flip the flag at runtime, and record compilation and
deoptimization events.

The post should make deoptimization feel less magical. Optimized machine code
contains assumptions. When an assumption fails, execution has to return to a
state the interpreter or a less-optimized version can understand. A feature flag
is a nice business-shaped example because it looks ordinary while poking a
serious runtime mechanism.

## 77. CPython GC cycle detection

I made two Python objects that point at each other, deleted the external names,
and checked why reference counting alone cannot free them. CPython's usual
memory management story is immediate and simple until cycles appear.

TODO: actually run this and check: use `gc` module introspection, examples with
containers, `__del__`, tracked and untracked objects, and read the relevant
CPython source.

The post should show the refcounts becoming useless. Each object keeps the other
alive, so neither reaches zero. The cycle collector has to find groups of
objects that are reachable from each other but not from the outside. That
sentence sounds simple, and the interesting part is the machinery CPython uses
to make it work without turning every object into a tracing-GC object.

## 78. CPython GC: why not just copy Java?

I tried to answer the deliberately annoying question: if the JVM has a good
moving generational garbage collector, why does CPython not use something like
that? The answer starts with CPython's C API, object identity, raw pointers,
reference counting, and decades of extension modules that expect objects to sit
where they are.

TODO: actually run this and check: read CPython C API constraints, examples of
borrowed references and object addresses, current GC/free-threading work, and
compare with JVM safepoints and moving collectors.

The post should not make this sound like Java people are clever and Python
people forgot to import the good collector. Runtime design is path dependence
made executable. CPython exposes object details to C extensions in ways the JVM
does not. A moving collector would have to either hide those details, update
them safely, or break code that assumes them.

## 79. C++ memory order

I wrote a few tiny atomic programs and compiled them for different targets. This
is the only way I trust myself to talk about C++ memory order, because the
abstract vocabulary becomes slippery very quickly. `relaxed`, `acquire`,
`release`, and `seq_cst` should be attached to code and, where possible,
assembly.

TODO: actually run this and check: implement counter, message-passing, spinlock,
and double-checked examples, compile for x86 and ARM, and record assembly and
runtime behavior where meaningful.

The post should resist explaining every memory order at once. Start with a
counter where relaxed is enough, then a message-passing example where ordering
matters, then a lock where acquire and release have a job. The conclusion can be
boring if the examples are good: use the weakest ordering only when you can
explain the proof without squinting.

## 80. Java scoped values vs Python context variables

I wrote the same request-context example in Java and Python: attach a request
ID, call through a few layers, cross an async or thread boundary, and see where
the value appears. Java scoped values and Python `contextvars` both solve the
problem of passing context without threading an argument through every function.

TODO: actually run this and check: implement examples using Java scoped values,
virtual threads if relevant, Python `contextvars`, asyncio tasks, threads, and
callbacks.

The post should include the case where context propagation fails or becomes
surprising. Dynamic context is useful because explicit parameters can become
tedious. It is dangerous because hidden inputs make code harder to inspect. The
comparison is useful only if it shows how each runtime decides which child task,
thread, or callback inherits the value.

