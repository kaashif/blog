# Explained blog post ideas

This is a working expansion of `blog-post-ideas.txt`.

The shape is deliberately practical: what the idea seems to mean, what I would
look up or try next, and a small three-part outline. The intended style is the
usual one: start with the odd concrete thing, ask what is really going on, then
look at real code, real data, real rules, or real binaries.

## 1. GCC extension for expression statements

GCC has statement expressions, written like `({ ... })`, which let a block of C
statements produce a value. This is exactly the kind of weird non-standard C
feature that explains how old-school macros get away with murder.

Next research:
- Find real examples in glibc, Linux, or common C libraries.
- Compare GCC and Clang support, and what MSVC does.
- Check the traps: lifetime, control flow, `typeof`, and C++ interaction.

Outline:
- Start with a macro that should be impossible in ordinary C.
- Expand it and show that GCC is treating a block as an expression.
- End with when this is useful, when it is cursed, and why portability loses.

## 2. MicroPython and CircuitPython

This is probably a comparison of two Python-like runtimes for microcontrollers.
The interesting question is not "can I run Python on a tiny board?" but what
had to be removed, faked, or redesigned to make that sentence true.

Next research:
- Pick one cheap board that runs both, and try the same small program.
- Compare supported libraries, I/O APIs, memory use, and deployment workflow.
- Find where they deliberately diverged in goals.

Outline:
- Start with a blinking LED or sensor script that looks suspiciously easy.
- Look at memory, imports, filesystem, and hardware APIs in both runtimes.
- Conclude with what kind of project each runtime is actually good for.

## 3. Large files make Git slow

Git is very good at source code and much less happy when it is used as a
general blob warehouse. Large files hit object storage, packfiles, delta
compression, clone size, checkout time, and history rewriting.

Next research:
- Create repos with many small files, a few large files, and modified large files.
- Measure clone, checkout, status, gc, and pack times.
- Compare plain Git with Git LFS and maybe partial clone.

Outline:
- Start with the innocent act of committing one giant binary.
- Measure which Git operations get worse and why.
- End with a concrete rule for when Git LFS or another store is not optional.

## 4. Hyperfine for profiling

`hyperfine` is a nice tool for timing command-line programs, but the post should
probably be about the difference between a useful benchmark and a decorative
number. Timing is easy. Measuring the thing you think you are measuring is not.

Next research:
- Benchmark a few tiny commands with and without warmups.
- Check shell startup, disk cache, CPU scaling, and variance.
- Compare with `time`, `perf stat`, and Criterion-style benchmark harnesses.

Outline:
- Start with two commands where the "faster" one changes after warmup.
- Show what `hyperfine` does well: runs, stats, export, parameter sweeps.
- End with a checklist for when CLI timing is enough and when it is not.

## 5. CWG 1331 and the `std::optional` mistake

This is about a C++ core language rule around defaulted copy constructors and
constness that made useful library types harder to write. `std::optional` is a
good hook because it exposes obscure object model rules through a type everyone
thinks is simple.

Next research:
- Read the CWG 1331 issue and the relevant standard wording.
- Find the smallest example where defaulted special members behave surprisingly.
- Check historical compiler behavior and any `optional` implementation notes.

Outline:
- Start with a tiny optional-like type that should obviously copy.
- Show the standard rule that says "not quite".
- Explain why library authors care and what changed.

## 6. C++ `std::error_code` and `<system_error>`

This is about C++'s standard error-code machinery: `std::error_code`,
`std::error_category`, `std::errc`, and `<system_error>`. It is simultaneously
reasonable, underused, and much more verbose than the average C programmer
would tolerate.

Next research:
- Build a tiny wrapper around POSIX errors using `std::error_code`.
- Compare exceptions, `errno`, `std::expected`, and `std::system_error`.
- Look at how Boost.Asio or filesystem APIs use it in practice.

Outline:
- Start with a failing syscall and the usual `errno` dance.
- Rebuild the example using `std::error_code` and categories.
- End with what problem the abstraction solves and why it still feels clunky.

## 7. GitHub Dependabot action for Maven auto-PRs

The idea is probably a practical post on making Java dependency updates less
manual. Maven dependency trees are annoying enough that "Dependabot opens a PR"
is only the first five percent of the story.

Next research:
- Configure Dependabot for a small Maven project.
- Add CI that runs tests and maybe groups harmless patch updates.
- Try a dependency with a transitive break and inspect the generated PR.

Outline:
- Start with a stale Maven dependency and the boring manual update process.
- Show the exact GitHub configuration and generated PR.
- End with the failure modes: noisy PRs, broken transitive deps, and trust.

## 8. `hardbool`

`hardbool` is about making boolean-like integer types stricter, so only the
intended true and false representations are valid. It is a small feature with a
large embedded-systems smell: booleans are not always just booleans.

Next research:
- Read the GCC/sourceware discussion and examples.
- Try compiling examples that put invalid bit patterns into the type.
- Compare with C23 `_Bool`, enums, bitfields, and sanitizer checks.

Outline:
- Start with a boolean stored in a byte that can secretly contain `42`.
- Show what `hardbool` promises to enforce.
- End with whether this is real safety or just another C extension with teeth.

## 9. Kafka is not a queue

Kafka can be used for queue-like jobs, but it is fundamentally a partitioned
append-only log with consumer offsets. Treating it as a normal work queue leads
to strange expectations about deletion, retries, ordering, and fanout.

Next research:
- Compare one job-queue example in Kafka, RabbitMQ, and a database table.
- Inspect consumer groups, partitions, retention, and replay.
- Look for real incidents caused by "Kafka as a queue" assumptions.

Outline:
- Start with a task queue design that uses Kafka because Kafka is already there.
- Show where queue semantics and log semantics disagree.
- End with when Kafka is the right tool and when it is just expensive confusion.

## 10. TLA+ or something similar in Haskell or OCaml

The idea is to model a small concurrent or distributed system and compare TLA+
with a typed functional implementation or model checker. The real question is
whether types make the model clearer, or whether the model checker is the point.

Next research:
- Pick a tiny protocol: lock service, retry queue, or request hedging.
- Model it in TLA+ first.
- Try an equivalent executable model in Haskell or OCaml.

Outline:
- Start with a simple system that has a non-obvious race.
- Write the smallest model that finds the bad state.
- Compare "spec language" clarity with "programming language" comfort.

## 11. V8 doubles on the heap vs CPython integers on the heap

This is a runtime representation post. JavaScript engines and CPython both
have to represent small values, object references, numbers, and dynamic types,
but they make different tradeoffs around boxing, tagging, and allocation.

Next research:
- Inspect V8 tagged values, SMIs, heap numbers, and NaN-boxing history.
- Inspect CPython `PyLongObject` layout and small integer caching.
- Write memory and speed experiments for arrays of numbers in JS and Python.

Outline:
- Start with the absurd sentence that an integer can be a heap object.
- Draw the actual layouts for JS numbers and Python integers.
- End with why "dynamic language" performance is mostly representation choices.

## 12. x64 assembler in Python with PeachPy

PeachPy is an assembler embedded in Python. The post can ask a nice concrete
question: why would anyone want to write assembly from Python instead of just
writing assembly?

Next research:
- Generate a small x86-64 function with PeachPy.
- Compare the generated assembly with handwritten assembly and compiler output.
- Try one SIMD or calling-convention-sensitive example.

Outline:
- Start with a Python file that emits real machine code.
- Show how it handles registers, ABI details, and instruction selection.
- End with whether embedded assemblers are a clever tool or too clever.

## 13. Write C to generate an ELF binary

This is a good "pull back the curtain" post: write a C program whose output is
an executable ELF file. The point is to make executable formats feel less like
magic and more like structs plus rules.

Next research:
- Build the smallest Linux x86-64 ELF that exits with a known code.
- Write a C generator for its headers and program segment.
- Inspect it with `readelf`, `objdump`, and the kernel loader behavior.

Outline:
- Start with `./a.out` and ask what the kernel actually reads.
- Generate the smallest working ELF by hand.
- End with the line between "file format" and "program" being thinner than it looks.

## 14. Locality of behaviour and spooky action at a distance

This is an architecture/design post, probably using htmx as the friendly
example. The point is that code is easier to reason about when behavior is near
the markup or data it affects, and harder when effects are routed through
hidden global conventions.

Next research:
- Read the htmx locality-of-behaviour post.
- Pick one small UI implemented in htmx and in a more state-heavy framework.
- Trace where one button click is handled in each version.

Outline:
- Start with a button whose behavior is impossible to find.
- Compare local attributes, controllers, stores, effects, and event buses.
- End with the tradeoff: local behavior is not always pure, but it is inspectable.

## 15. Folly `CoreCachedSharedPtr`

This sounds like a post about reducing shared-pointer reference-count
contention by caching per core. Shared ownership looks cheap until many threads
hammer the same atomic counter.

Next research:
- Read Folly's implementation and intended usage.
- Build a benchmark with many threads copying the same `shared_ptr`.
- Compare ordinary `shared_ptr`, raw pointer plus lifetime discipline, and Folly.

Outline:
- Start with a `shared_ptr` that gets slower when "all" you do is copy it.
- Show the refcount traffic and why cores fight over it.
- End with what per-core caching buys and what complexity it hides.

## 16. Find a network driver to port to OpenBSD

This is very much in the existing blog style: pick an old or simple network
driver and try to port it to OpenBSD. The post is the journey through driver
APIs, not the triumphant announcement.

Next research:
- Identify a simple NIC with available docs and another BSD/Linux driver.
- Read OpenBSD driver examples for similar hardware.
- Work out bus, DMA, interrupt, and network stack interfaces.

Outline:
- Start with a card that OpenBSD does not support, or supports badly.
- Walk through the source of a working driver elsewhere.
- End with a list of what made the port hard before claiming success.

## 17. Undo / time-travel debugging

Reversible debugging sounds like a superpower, but the post should ask what has
to be recorded to let execution run backwards. The useful comparison is `rr`,
Undo, and ordinary logging/core dumps.

Next research:
- Try a small nondeterministic bug under `rr`.
- Read how record/replay handles syscalls, signals, scheduling, and ASLR.
- Compare commercial Undo-style tooling with open-source `rr`.

Outline:
- Start with a bug where the bad state is visible only after the cause is gone.
- Replay it backwards and inspect what information was recorded.
- End with why reversible debugging is amazing and still not always used.

## 18. Haskell future monad vs Reactor Java

This is a comparison of async composition styles: typed functional effects on
one side, Java reactive streams on the other. The interesting parts are
cancellation, errors, backpressure, and how much the type system helps.

Next research:
- Implement the same three async operations in Haskell and Reactor.
- Add failure, cancellation, timeout, and bounded concurrency.
- Compare the generated stack traces and debugging experience.

Outline:
- Start with a small service fanout that looks simple in pseudocode.
- Write it using a Haskell future/async abstraction and Java Reactor.
- End with where each model becomes annoying.

## 19. Java 21 ADTs from XSD `choice`

This is about converting XML Schema `choice` structures into modern Java sum
types. Java 21 sealed classes, records, and pattern matching make this much less
awful than the old "one nullable field per possibility" style.

Next research:
- Find a small XSD with nested `choice`.
- Generate normal JAXB-style code and inspect the awkwardness.
- Prototype sealed interfaces and records for the same schema.

Outline:
- Start with an XSD `choice` that generates a bad Java API.
- Show the modern Java representation as a real sum type.
- End with what codegen would have to do to make this usable.

## 20. Heap after GC as a missing Micrometer feature

This is a JVM observability post. Current heap usage is noisy; heap after a GC
is closer to the live set and often a better signal for memory pressure. The
idea is that Micrometer should expose this cleanly.

Next research:
- Check what Micrometer and JVM MXBeans expose today.
- Listen to GC notifications and record post-GC heap size.
- Compare the metric against normal heap-used graphs during a leak.

Outline:
- Start with a heap graph that looks scary but tells you little.
- Show why post-GC heap is the number you actually wanted.
- End with a small Micrometer implementation or contribution sketch.

## 21. Pre-Java Oak

This is about Oak before it became Java: the prehistoric language, its syntax,
goals, libraries, and what changed before the public Java 1.0 story. The hook is
that Java did not emerge fully formed as enterprise beige.

Next research:
- Find early Oak manuals, specs, examples, or archived source.
- Compare Oak syntax and APIs with Java 1.0.
- Identify which features were removed, renamed, or carried forward.

Outline:
- Start with an early Oak code sample that looks almost, but not quite, like Java.
- Walk through what Oak was for before the web-applet story took over.
- End with what survived into Java and what got buried.

## 22. Windows ABI vs Itanium/System V struct splitting

This is about how C++ ABIs pass and return aggregates. Small structs can be
split across registers, passed indirectly, or classified by strange rules, and
the rules differ between Microsoft x64 and Itanium/System V worlds.

Next research:
- Compile tiny functions taking structs of different shapes on MSVC and Clang/GCC.
- Inspect assembly and ABI documents.
- Include C++ cases with constructors, non-trivial copy, and inheritance.

Outline:
- Start with a function taking `{ int, double }` and ask where the bits go.
- Compare the ABI classifications on two platforms.
- End with why FFI and binary compatibility are not "just call the function".

## 23. Futex

Futexes are the Linux primitive behind many user-space locks: try the fast path
in user space, ask the kernel to sleep only when contention happens. The
interesting thing is how little the kernel needs to know.

Next research:
- Implement a toy mutex using `futex`.
- Compare uncontended and contended paths.
- Read how pthread mutexes map onto futex operations.

Outline:
- Start with the complaint that a mutex syscall for every lock would be stupid.
- Build the minimal futex-based lock.
- End with the hard parts: wakeups, races, priority inheritance, and robustness.

## 24. Kotlin coroutines vs reactive for limiting concurrency

This is about bounded concurrency in Kotlin: do you use coroutines with a
semaphore/dispatcher, or reactive streams with backpressure? The answer is less
about fashion and more about what failure and cancellation look like.

Next research:
- Implement "fetch 10,000 URLs with at most 50 in flight" both ways.
- Add timeouts, retries, and partial failure.
- Measure code complexity and runtime behavior.

Outline:
- Start with an accidentally unbounded fanout.
- Compare coroutine and reactive implementations.
- End with which model makes the dangerous part obvious.

## 25. Kryo serialization: how does it work?

Kryo is a fast Java binary serialization library. The useful post would explain
what "fast serialization" actually means: class IDs, object graphs, reference
tracking, field layout, and compatibility hazards.

Next research:
- Serialize a few simple and cyclic object graphs.
- Inspect bytes with and without class registration.
- Compare speed and compatibility with Java serialization and Jackson.

Outline:
- Start with a Java object graph and a surprisingly small byte array.
- Decode what Kryo wrote and why registration matters.
- End with the tradeoff between speed, schema stability, and footguns.

## 26. BetaArchive and Internet Explorer for Solaris

This is retro software archaeology. Internet Explorer for Solaris existed, and
the post should probably be about finding it, running it, and seeing what kind
of Unix desktop world Microsoft expected.

Next research:
- Find legitimate archived installers or documentation.
- Identify the Solaris version and architecture needed.
- Try running it in a VM or on old Sun hardware if possible.

Outline:
- Start with the absurd phrase "Internet Explorer for Solaris".
- Get the software into an environment where it can actually launch.
- End with what it reveals about the late-90s browser/platform war.

## 27. Apache Tika for metadata extraction

Tika is a huge "read text and metadata from almost anything" library. The
interesting post is not just how to call it, but how much trust you should put
in a universal parser.

Next research:
- Feed Tika PDFs, Office docs, images, audio files, and weird corrupt files.
- Compare extracted metadata with `exiftool`, `pdfinfo`, and manual inspection.
- Check sandboxing and parser security guidance.

Outline:
- Start with a pile of files and the desire for one parser.
- Show what Tika extracts correctly and what it gets weird.
- End with how to use it without pretending all formats are sane.

## 28. Graal and Truffle: write a BASIC interpreter

This is a perfect "no magic" compiler/runtime post. Implement a small BASIC in
Truffle and show how Graal can optimize an AST interpreter if you follow the
framework's rules.

Next research:
- Define a tiny BASIC subset: `LET`, `PRINT`, `GOTO`, maybe `FOR`.
- Implement it first as a boring interpreter, then with Truffle nodes.
- Inspect warmup, specialization, and performance.

Outline:
- Start with a BASIC program short enough to fit in the post.
- Build the interpreter and show where Truffle wants structure.
- End with what Graal optimizes and what remains ordinary interpreter work.

## 29. Winning strategy in QE

QE is the board game about quantitative easing, so this is likely a strategy
and valuation post in the Sidereal Confluence style. The core question is how
to price bids when money is both unconstrained and dangerous.

Next research:
- Read the rules and scoring carefully.
- Model simplified auctions and penalty mechanics.
- Collect or simulate sample games to test bidding heuristics.

Outline:
- Start with the weird rule: you can bid any amount of money.
- Turn the scoring and penalty structure into a small model.
- End with practical bidding rules and where the model is probably wrong.

## 30. Do not use patching in Python

This is an anti-monkeypatching and anti-`mock.patch` testing post. The claim is
not that patching never works, but that it lets tests lie about the shape of the
program.

Next research:
- Build a tiny example where `patch` hides a bad dependency boundary.
- Rewrite it with dependency injection, fakes, or explicit parameters.
- Check where patching is still the least bad option.

Outline:
- Start with a passing test that proves almost nothing.
- Show how patching changes global lookup rather than design.
- End with a rule of thumb for tests that survive refactors.

## 31. JSON vs Arrow serialization/deserialization speed

This is about row-oriented text versus typed columnar memory. JSON is flexible
and everywhere; Arrow is structured, compact, and often closer to the shape that
analytics code wants.

Next research:
- Choose one dataset with numbers, strings, nullable fields, and timestamps.
- Benchmark parse, write, read, memory use, and filtering.
- Include Python and maybe Rust or Java bindings.

Outline:
- Start with the same data in JSON and Arrow.
- Measure the cost of turning bytes into useful values.
- End with when Arrow wins and when JSON's stupidity is a feature.

## 32. Atomics on x86 vs SPARC

This fits the old hardware/architecture line: atomic operations are not a
single universal thing. x86 and SPARC differ in memory model, instruction set,
barriers, and the cost of making ordering promises.

Next research:
- Read x86 and SPARC atomic/barrier docs.
- Compile C++ atomic examples for both targets.
- If possible, run or emulate simple benchmarks.

Outline:
- Start with the same `std::atomic` code compiled for two architectures.
- Inspect the instructions and barriers each target needs.
- End with why C++ memory order exists and why hardware still leaks through.

## 33. Elixir and crashing often as error handling

This should be about the Erlang/Elixir "let it crash" philosophy: isolate work
in processes, supervise failures, and recover by restarting rather than trying
to handle every local error.

Next research:
- Build a tiny Elixir supervision tree with a deliberately flaky worker.
- Compare with Java/Python-style local exception handling.
- Read how OTP supervisors choose restart strategies.

Outline:
- Start with a process that randomly crashes.
- Show why the system stays alive if failure boundaries are right.
- End with what "let it crash" does not excuse.

## 34. Annotation processor for compile-time XSD

This overlaps with the Java 21 XSD idea, but the angle here is build-time
codegen. Annotation processing can turn schema declarations into Java types
before runtime, which is nicer than reflection-heavy XML magic.

Next research:
- Prototype an annotation that points at an XSD file.
- Generate sealed classes/records or validators during compilation.
- Check incremental compilation and IDE behavior.

Outline:
- Start with a runtime XML parser finding a schema mistake too late.
- Move the schema into compile-time code generation.
- End with what Java annotation processing makes easy and what it makes horrible.

## 35. Transcript search with Rust and embedded SQLite

This is a revisit of transcript search using Rust and embedded SQLite FTS. The
question is what you gain by replacing a server database with a single-file
index and a small binary.

Next research:
- Convert a small transcript corpus into SQLite FTS5.
- Build a Rust CLI or tiny web server around it.
- Compare indexing time, query quality, deployment, and file size.

Outline:
- Start with the old transcript-search setup and what annoyed you.
- Build the embedded version and show the schema.
- End with whether "just SQLite" is enough.

## 36. Shell scripts as an escape hatch out of unit tests

Shell scripts often become the place where the real system behavior lives after
the "proper" code ends. They are easy to write, hard to test, and excellent at
making CI lie.

Next research:
- Find or construct a build/deploy script with hidden important logic.
- Add tests with `bats` or shellcheck and see what is still hard.
- Compare moving logic into a typed/tested language.

Outline:
- Start with a one-line shell script that quietly becomes production logic.
- Show the ways it escapes normal test boundaries.
- End with when shell is fine and when it is where correctness goes to hide.

## 37. Compression algorithm implementation

This is a "write the thing yourself" post. Implement a small compression
algorithm so the terms stop being mysterious: dictionary, entropy coding,
window, literals, matches, and headers.

Next research:
- Pick one algorithm small enough for a blog post: Huffman, LZ77, or LZSS.
- Implement compressor and decompressor.
- Test on text, binaries, repeated data, and random data.

Outline:
- Start with a string that obviously has redundancy.
- Implement the smallest useful compressor and inspect its output.
- End with why real formats add so much machinery.

## 38. Performant rounding

Rounding sounds boring until you care about speed, exactness, and edge cases.
The post could cover float-to-int rounding, decimal rounding, vectorization,
and why "just add 0.5" is nonsense.

Next research:
- Decide which rounding problem: float to int, decimal money, or batch rounding.
- Compare C/C++ standard functions, CPU instructions, and custom code.
- Test negative values, halfway cases, NaNs, overflow, and locale/decimal issues.

Outline:
- Start with the classic wrong rounding trick.
- Benchmark correct alternatives and inspect assembly.
- End with the smallest rule set that avoids embarrassing numerical bugs.

## 39. Rust `bumpalo`

`bumpalo` is arena allocation for Rust: allocate many objects quickly, then free
them all at once by dropping the arena. It is fast because it gives up general
individual deallocation.

Next research:
- Implement a parser or tree builder with and without `bumpalo`.
- Measure allocation count, time, and memory behavior.
- Explain lifetimes and why arena-owned values cannot casually escape.

Outline:
- Start with a program allocating thousands of tiny nodes.
- Replace normal allocation with a bump arena and measure.
- End with the lifetime discipline that makes the speed honest.

## 40. Rust audio synthesizer latency

This is about building a tiny synth and discovering that real-time audio is not
ordinary application programming. Allocation, locks, scheduling, buffer sizes,
and GC-like pauses all matter.

Next research:
- Build a simple oscillator with `cpal` or another Rust audio crate.
- Measure latency and glitches under CPU load.
- Try adding UI/control messages without blocking the audio callback.

Outline:
- Start with a sine wave that should be trivial.
- Show what goes wrong when the callback misses deadlines.
- End with the rules for writing code that lives on the audio thread.

## 41. Write a gRPC service

This is probably a practical gRPC post, but it should avoid being a tutorial
that says "install protoc". The more interesting thing is what gRPC commits you
to: protobuf schemas, HTTP/2, generated code, streaming, deadlines, and status
codes.

Next research:
- Build one tiny service with unary and streaming methods.
- Inspect generated code and on-the-wire behavior.
- Add deadlines, cancellation, and an error case.

Outline:
- Start with a normal JSON endpoint and the desire for typed clients.
- Implement the equivalent gRPC service.
- End with what gRPC makes better and what it makes more annoying.

## 42. How Java objects are laid out in memory

This is a JVM internals post using actual object layouts. Headers, compressed
oops, alignment, field packing, arrays, inheritance, and locks all show up in
ordinary-looking Java code.

Next research:
- Use JOL on small classes, arrays, strings, and nested objects.
- Toggle compressed oops if possible.
- Compare expected layout with measured layout.

Outline:
- Start with a Java class and ask how many bytes it really costs.
- Use JOL to expose headers, fields, padding, and references.
- End with why Java memory use is not just "sum of field sizes".

## 43. GPTs to make a biography of your parents

This is a non-systems post, but it can still be concrete: interview people,
transcribe the audio, extract chronology, and generate a draft biography. The
risk is fake sentimentality and hallucinated family history.

Next research:
- Design interview questions and consent/privacy rules.
- Test transcription, summarization, and chronology extraction.
- Compare AI-generated prose with direct edited transcript excerpts.

Outline:
- Start with the fear that family stories vanish if nobody writes them down.
- Build a workflow from interview audio to structured notes to draft.
- End with what an LLM is allowed to invent, which is basically nothing.

## 44. Bubblewrap containers

`bubblewrap` is a small Linux sandboxing tool built around namespaces and bind
mounts. It is a nice way to show that containers are not magic; they are kernel
features with a command-line interface.

Next research:
- Create a minimal filesystem sandbox with `bwrap`.
- Add network isolation, read-only mounts, and a writable temp directory.
- Compare with Docker for the same tiny command.

Outline:
- Start with running `/bin/sh` in a filesystem that should not exist.
- Add namespaces and mounts one at a time.
- End with what bubblewrap gives you and what it very much does not.

## 45. Polonius borrow checker

Polonius is an experimental Rust borrow checker model intended to make borrow
analysis more precise. The hook is examples that humans think are fine but the
current borrow checker rejects.

Next research:
- Find canonical Polonius examples and current compiler behavior.
- Read the high-level datalog/loan analysis explanation.
- Try nightly flags if still available.

Outline:
- Start with a tiny Rust program that looks safe but is rejected.
- Explain what the current borrow checker cannot prove.
- End with what Polonius changes and why it is hard to ship.

## 46. `Libfn/functional`

This appears to be a C++ functional-programming helper library. The interesting
question is whether monadic combinators around `optional`, `expected`, and
friends make C++ clearer, or just make it cosplay as Haskell.

Next research:
- Identify the exact repo and its current API.
- Rewrite a small error-handling pipeline with and without it.
- Compare with C++23 library additions.

Outline:
- Start with nested `if` statements around optional/error values.
- Replace them with functional combinators.
- End with whether the abstraction removes complexity or moves it into names.

## 47. `uv` being much faster than `pip`

This is a benchmarking and implementation post about Python packaging. `uv` is
fast enough compared with `pip` that the interesting question is where the time
went in the old model.

Next research:
- Benchmark fresh install, cached install, resolver-only, and lockfile flows.
- Use projects with small and ugly dependency graphs.
- Read enough implementation notes to separate Rust from algorithmic changes.

Outline:
- Start with installing the same requirements using `pip` and `uv`.
- Break the time down into resolving, downloading, building, and installing.
- End with what speed changes about everyday Python development.

## 48. Rust destructors are not guaranteed

Rust's `Drop` is deterministic in normal control flow, but it is not a
mathematical promise that cleanup always happens. Leaks, `mem::forget`, cycles,
abort, and process exit all matter.

Next research:
- Build examples for `mem::forget`, `Rc` cycles, panic abort, and process exit.
- Compare with C++ RAII expectations.
- Check what unsafe code may and may not assume about `Drop`.

Outline:
- Start with a `Drop` impl printing "cleanup" and make it not print.
- Catalogue the ordinary ways destructors are skipped.
- End with what Rust actually guarantees and what people casually assume.

## 49. Rust static reflection drama vs C++ reflection

This is about compile-time introspection and metaprogramming politics. Rust has
proc macros and trait machinery; C++ has a long history of template misery and
static reflection proposals. Both communities want power without nonsense.

Next research:
- Identify the specific Rust static reflection discussion you had in mind.
- Compare a small serialization/codegen task in Rust and proposed C++ reflection.
- Read current C++ reflection proposal status.

Outline:
- Start with a boring task: derive field names and types.
- Show how Rust and C++ try to solve it.
- End with what each language is afraid of becoming.

## 50. Codon as faster Python with a JIT/compiler

Codon promises Python-like code with much better performance by compiling a
subset/variant of Python. The useful post should test the boundary between
"Python, but fast" and "a different language with familiar syntax".

Next research:
- Run numeric loops, object-heavy code, and library-using Python code.
- Compare CPython, PyPy if relevant, Numba, and Codon.
- Identify unsupported or semantically different Python features.

Outline:
- Start with a Python loop everyone says is slow.
- Run it through Codon and inspect what changed.
- End with where Codon is impressive and where "Python compatible" gets fuzzy.

## 51. Hazard pointers

Hazard pointers solve a specific lock-free data structure problem: how do you
free a node when another thread might still hold a raw pointer to it? Readers
publish the pointers they are currently using.

Next research:
- Implement or read a small lock-free stack with unsafe reclamation.
- Add hazard pointers and show the memory lifecycle.
- Compare with epoch reclamation and reference counting.

Outline:
- Start with a lock-free pop that returns a pointer to memory someone frees.
- Show how a hazard pointer prevents that exact bug.
- End with why memory reclamation is often the real lock-free problem.

## 52. Varargs C++ functions cannot be coroutines

C++ coroutine transformation needs to preserve parameters in a coroutine frame.
C-style varargs are not a friendly thing to preserve, type, copy, or resume
around, so the standard forbids this combination.

Next research:
- Compile examples with `...` and `co_await` and capture diagnostics.
- Read the standard restriction and rationale if available.
- Compare C varargs, variadic templates, and coroutine frame layout.

Outline:
- Start with a function that is both variadic and wants to suspend.
- Show the compiler saying no and inspect why the frame would be awkward.
- End with the cleaner alternative: variadic templates or explicit arguments.

## 53. Rewinding C++ coroutines with memory trickery

This is intentionally implementation-dependent and undefined-behavior-heavy:
can you mutate the coroutine frame's saved state to resume from somewhere else?
It pairs well with the existing coroutine decompilation post.

Next research:
- Start from a minimal coroutine and inspect frame layout for one compiler/build.
- Find the state field and try mutating it before `resume`.
- Repeat across optimization levels and compilers to show it is not portable.

Outline:
- Start with the question "what if I edit the coroutine's save slot?"
- Demonstrate the smallest possible cursed state jump.
- End with why this works only as archaeology, not as programming advice.

## 54. Python 10m threads: no-GIL vs async

This is about testing the practical meaning of free-threaded/no-GIL Python
against async I/O. Ten million threads is probably not a realistic target, but
that is exactly why it is a good stress question.

Next research:
- Check current free-threaded Python builds and thread memory overhead.
- Compare many sleeping tasks in threads, asyncio, and maybe gevent.
- Separate CPU parallelism from I/O concurrency.

Outline:
- Start with the claim that no-GIL makes threads the obvious answer.
- Measure thread count, memory, scheduling, and throughput.
- End with what no-GIL changes and what it does not.

## 55. `sysret_ss_attrs` AMD64 design mistake

This is a CPU/OS bug archaeology post around `SYSRET` and segment selector
state on AMD64. The attractive part is that modern 64-bit systems still carry
weird segmentation baggage.

Next research:
- Identify the exact AMD64 issue and kernel workaround.
- Read Linux, BSD, or OSDev explanations.
- Create the smallest explanation of `SYSCALL`/`SYSRET` state transitions.

Outline:
- Start with the fact that 64-bit mode still has segment selectors.
- Follow a syscall return path and show where SS attributes matter.
- End with why one architectural shortcut turns into OS workaround code forever.

## 56. Linux `readv` and `writev`

Scatter/gather I/O lets one syscall read into or write from multiple buffers.
It is a simple API that becomes interesting when you look at syscall overhead,
buffer copying, and protocol framing.

Next research:
- Write examples that send headers plus body with `writev`.
- Benchmark against concatenating into one buffer and calling `write`.
- Check limits, partial writes, and interaction with sockets.

Outline:
- Start with code that mallocs a temporary buffer just to add a header.
- Replace it with `writev`.
- End with the practical rules around partial writes and buffer lifetimes.

## 57. Tearing of 64-bit Java types

The Java memory model historically allowed non-volatile `long` and `double`
reads/writes to tear into two 32-bit halves. Most modern 64-bit JVMs do the
reasonable thing, but the spec history is the interesting bit.

Next research:
- Read the current JLS wording for non-volatile `long`/`double`.
- Try to reproduce tearing on old or 32-bit JVMs if possible.
- Compare with `volatile`, atomics, and VarHandles.

Outline:
- Start with a `long` value that could theoretically become nonsense.
- Explain tearing and the memory model rule.
- End with what matters today and why specs sometimes keep old scars.

## 58. Is Tokio WASM-compatible?

Tokio assumes a runtime environment with timers, I/O, tasks, and sometimes
threads. WebAssembly environments vary wildly, so "Tokio on WASM" depends on
whether you mean browser WASM, WASI, or something else.

Next research:
- Try a tiny Tokio future in browser WASM and WASI.
- Check which features require OS sockets, threads, or timers.
- Compare with `wasm-bindgen-futures` and async executors built for WASM.

Outline:
- Start with a Rust async program that works on Linux.
- Compile it to WASM and watch which assumptions break.
- End with a compatibility table by environment and feature.

## 59. Is pip's dependency solver Turing-complete?

This is a fun complexity post. Dependency resolution is at least SAT-like in
spirit, and package constraints can encode nasty search problems. The question
is whether "Turing-complete" is true, false, or just a good joke.

Next research:
- Read pip's resolver model and version constraint language.
- Try encoding SAT or another hard problem using package versions.
- Be precise about decidable search vs Turing completeness.

Outline:
- Start with pip apparently thinking very hard about package versions.
- Build the smallest evil dependency graph.
- End with the correct complexity claim, even if it ruins the joke.

## 60. `rkyv` for zero-copy deserialization

`rkyv` archives Rust data into a layout that can be read without building a new
object graph. The interesting part is what has to be true for bytes on disk to
be safely treated as structured data.

Next research:
- Archive a few structs and inspect the byte layout.
- Benchmark against `serde_json`, `bincode`, and postcard.
- Check validation, alignment, endianness, and versioning.

Outline:
- Start with "deserialization" that does almost no work.
- Show the archived representation and the safety checks.
- End with where zero-copy is worth the constraints.

## 61. Rust profiling: `hyperfine` and Codespeed

This is about the ladder from quick local timings to tracked performance over
time. `hyperfine` answers "is this faster on my machine now?"; Codespeed-style
infrastructure answers "did this commit make it slower?"

Next research:
- Benchmark one Rust CLI with `hyperfine`.
- Add criterion or another microbenchmark where appropriate.
- Sketch a CI performance dashboard and noise controls.

Outline:
- Start with a Rust change that feels faster.
- Measure locally, then show why local timing is not enough.
- End with the minimum setup that catches performance regressions without lying.

## 62. Python `-O` optimization/debug flag

Python's `-O` flag is badly named if you expect a compiler optimizer. It mostly
changes `assert`, `__debug__`, and with `-OO`, docstrings. That is not the same
thing as making code fast.

Next research:
- Run examples showing `assert`, `__debug__`, and docstring behavior.
- Benchmark code with and without `-O`.
- Search real bugs caused by relying on assertions for runtime checks.

Outline:
- Start with someone running `python -O` expecting speed.
- Show exactly what bytecode or behavior changes.
- End with why `assert` is not validation.

## 63. ARM Jazelle: JVM bytecode on the CPU

Jazelle was ARM's attempt to execute Java bytecode in hardware or with hardware
assistance. It is a great historical post because it sounds absurd now but made
sense in a world of feature phones and Java ME.

Next research:
- Find public ARM Jazelle docs and old device references.
- Work out what was actually hardware-executed versus trapped/emulated.
- Compare with modern JVM JIT expectations.

Outline:
- Start with the claim that a CPU can run Java bytecode directly.
- Explain what Jazelle did and why mobile vendors cared.
- End with why the idea faded.

## 64. `sockaddr` fake flexible array

The socket API uses `struct sockaddr` as a generic envelope for many differently
sized address structures. This is old C ABI design: cast pointers, pass lengths,
and trust everyone to know what shape is really there.

Next research:
- Inspect `sockaddr`, `sockaddr_in`, `sockaddr_in6`, and `sockaddr_storage`.
- Trace a `bind` or `connect` call through headers and man pages.
- Explain historical fixed-size fields versus modern flexible arrays.

Outline:
- Start with casting `sockaddr_in *` to `sockaddr *` and pretending that is fine.
- Show the structs and the length argument doing the real work.
- End with why old C APIs look like type crimes.

## 65. SJLJ exceptions and C++ exception performance

Setjmp/longjmp exceptions and table-based "zero-cost" exceptions make different
tradeoffs. The interesting post is why exception performance claims depend so
much on implementation, architecture, and whether you mean throwing or not
throwing.

Next research:
- Compile the same C++ exception examples with SJLJ and table-based mechanisms if possible.
- Measure normal path and throw path.
- Inspect generated assembly and unwind tables.

Outline:
- Start with the phrase "zero-cost exceptions" and ask zero for whom.
- Compare mechanisms and where the cost is paid.
- End with a practical reading of exception benchmarks.

## 66. gRPC request hedging benchmarks

Request hedging sends backup requests after a delay to reduce tail latency. It
can improve the high percentiles while increasing load, duplicating work, and
making overload worse if configured stupidly.

Next research:
- Build a service with controlled latency distribution and occasional stragglers.
- Benchmark no hedging, immediate duplicates, and delayed hedges.
- Track p50/p95/p99, extra requests, cancellation timing, and server load.

Outline:
- Start with one slow request ruining p99 latency.
- Add hedging and show the distribution change, not just the average.
- End with the cost curve: better tails, more work, and possible self-harm.

## 67. Conservative GC: do not keep track of references

Conservative garbage collectors scan memory for things that look like pointers
instead of knowing exactly where every reference is. This makes C/C++ possible
to collect, but false positives and non-moving collection fall out naturally.

Next research:
- Read Boehm GC basics.
- Build a small C example where an integer-looking value keeps memory alive.
- Compare with precise moving GCs in managed runtimes.

Outline:
- Start with a GC that does not really know what a pointer is.
- Show how stack scanning and false positives work.
- End with why conservative GC is useful and inherently compromised.

## 68. `std::inplace_vector` and L1 cache

`std::inplace_vector` stores elements inline up to a fixed capacity. That can
avoid heap allocation and improve locality, but it also makes objects larger
and capacity part of the type/design.

Next research:
- Check the current C++ standard/library status for `inplace_vector`.
- Benchmark small vectors in tight loops against `std::vector`.
- Inspect object size, cache behavior, and move/copy costs.

Outline:
- Start with a vector of a few items allocating anyway.
- Replace it with inline storage and measure the cache/allocation effects.
- End with when fixed capacity is an advantage rather than a nuisance.

## 69. Double-wide atomic instructions: are they good yet?

This is about 128-bit or pairwise atomic compare-and-swap operations. They are
useful for lock-free structures because you often want to update pointer plus
counter together, but support varies across hardware, compilers, and alignment.

Next research:
- Test `std::atomic<__int128>` or platform intrinsics on x86-64 and ARM64.
- Check whether operations are actually lock-free.
- Build a tiny ABA-resistant stack or tagged pointer example.

Outline:
- Start with wanting to atomically update two machine words.
- Check what the compiler and CPU really emit.
- End with whether double-wide atomics are a practical primitive now.

## 70. AVX-512 Galois-field instructions

AVX-512 GFNI instructions do Galois-field affine transforms and multiplication.
They sound wildly specialized, but they connect to crypto, checksums, erasure
coding, and bit-matrix tricks.

Next research:
- Read the GFNI instruction descriptions.
- Implement a small GF(2^8) transform or AES-adjacent operation.
- Benchmark scalar, table-based, and GFNI versions if hardware is available.

Outline:
- Start with the existence of a CPU instruction no normal person asked for.
- Explain the finite-field operation in concrete byte terms.
- End with where the instruction is actually useful.

## 71. McKinsey space travel

This is about startups run by former McKinsey/Stanford MBA types, with "space
travel" as the exaggerated frontier-market version of the pattern. The likely
angle is sociology plus evidence: what these people claim, what they actually
build, and how the pitch language behaves.

Next research:
- Identify a few concrete startups and founder backgrounds.
- Read decks, websites, job ads, and customer claims.
- Separate legitimate hard-tech execution from consultant-flavored theatre.

Outline:
- Start with a startup website that sounds like a McKinsey slide learned rocket science.
- Look at founders, claims, product, funding, and actual technical output.
- End with what the MBA/operator pattern is good at and where it becomes nonsense.

## 72. Review of IM-1776 issue on health

The issue is IM-1776 Issue VI, "Health of a Nation". The site describes it as a
print issue about America's health crisis, with essays on RFK Jr.-adjacent
politics, medical institutions, relationships, vigor, farms, centralized
medicine, and the physical body.

Next research:
- Get the issue text or print copy and read the whole thing.
- Identify the factual claims that are testable rather than just vibes.
- Decide whether the review is political, medical, literary, or all three.

Outline:
- Start with what kind of magazine issue this is trying to be.
- Review the strongest and weakest essays by claim quality, not tribal alignment.
- End with what it gets right, what it handwaves, and what is just aesthetics.

## 73. `nanobind` for Python bindings

`nanobind` is a newer C++ binding library for Python, in the same broad world as
pybind11. The hook is whether it makes extension modules smaller, faster, or
less annoying.

Next research:
- Bind the same tiny C++ library with pybind11 and nanobind.
- Compare build setup, binary size, import time, call overhead, and docs.
- Check NumPy support and lifetime/reference semantics.

Outline:
- Start with a C++ function you want to call from Python.
- Build two bindings and inspect the generated extension behavior.
- End with whether nanobind is a drop-in improvement or just a different tradeoff.

## 74. Python performance tips, function calls, loops, CPython vs Codon

This is a myth-testing post. Python advice often says "avoid function calls" or
"never write loops", but the correct answer depends on interpreter overhead,
library boundaries, data size, and whether you changed language semantics.

Next research:
- Collect common Python performance tips and turn each into a benchmark.
- Compare CPython, PyPy if useful, and Codon.
- Include disassembly or profiler output for the surprising cases.

Outline:
- Start with a smug Python performance tip that sounds true.
- Benchmark it in enough contexts to make the slogan fall apart.
- End with rules that survive contact with actual measurements.

## 75. Python GIL can release in the middle of `+=`

The interesting claim is that apparently simple Python operations are not
atomic semantic units. `x += y` can call user code, allocate, run C extension
logic, or otherwise hit places where the GIL story is less comforting than it
looks.

Next research:
- Write classes whose `__iadd__` or `__add__` does surprising things.
- Check where CPython may release the GIL during allocation or C calls.
- Compare with bytecode-level "atomic" folklore.

Outline:
- Start with `counter += 1` looking like one operation.
- Break it apart into bytecode and method calls.
- End with the actual rule: the GIL is not a data-race design strategy.

## 76. Java JIT deoptimization when a feature flag flips

Modern JVMs optimize speculatively. If a branch has always gone one way, the JIT
may compile for that world; when the world changes, deoptimization throws
execution back to a safer representation.

Next research:
- Write a Java benchmark with a stable branch controlled by a mutable flag.
- Use JIT logging/JITWatch to observe compilation and deoptimization.
- Try class-loading or type-profile examples too.

Outline:
- Start with a feature flag that looks like ordinary business logic.
- Show the JIT assuming the hot path and then being proven wrong.
- End with why deoptimization is not a bug, but can surprise benchmarks.

## 77. CPython GC cycle detection

CPython mostly uses reference counting, but cycles need a separate collector.
The post should explain which objects are tracked, how generations work, and
how unreachable cycles are found.

Next research:
- Build examples with simple refcounts, cycles, `__del__`, and containers.
- Inspect `gc.get_objects`, `gc.is_tracked`, and collection stats.
- Read enough CPython source to explain the algorithm honestly.

Outline:
- Start with two objects that keep each other alive forever.
- Show why refcounting alone cannot solve it.
- End with the cycle collector's actual mechanism and its compromises.

## 78. CPython GC: why not just copy Java?

The answer is mostly the C API, object identity, raw pointers, reference
counting assumptions, extension modules, and compatibility. A moving
generational GC is not something you paste into CPython after lunch.

Next research:
- List the CPython object model constraints that prevent easy moving GC.
- Compare with JVM object references and safepoints.
- Read current/free-threading or immortal-object work that changes constraints.

Outline:
- Start with the reasonable question: Java's GC is good, why not use that?
- Show the CPython invariants and extension API assumptions.
- End with what would have to break to make moving GC plausible.

## 79. C++ memory order

This is the classic subject where almost every explanation becomes either too
abstract or too wrong. The way to do it in your style is with tiny programs,
compiled assembly, and only enough model theory to avoid lying.

Next research:
- Pick a few patterns: counter, message passing, spin lock, double-checked init.
- Compile for x86 and ARM if possible.
- Show where relaxed, acquire/release, and seq_cst differ.

Outline:
- Start with an atomic program that "obviously" works until it does not.
- Walk through memory orders using one concrete pattern at a time.
- End with a small set of rules for writing boring correct atomics.

## 80. Java scoped values vs Python context variables

Java scoped values and Python `contextvars` both address context propagation:
request IDs, auth info, tracing state, and other values you do not want to pass
through every function by hand. The details differ around mutability, threads,
tasks, and dynamic scope.

Next research:
- Build the same request-context example in Java and Python.
- Test propagation across threads, virtual threads, async tasks, and callbacks.
- Compare with thread locals and explicit parameters.

Outline:
- Start with a request ID that vanishes halfway through an async call.
- Implement the context using Java scoped values and Python context variables.
- End with when dynamic context is clean and when it becomes hidden global state.

## Remaining things to pin down later

- For `Libfn/functional`, identify the exact repository intended by the Slack
  note before writing anything public.
- For Rust static reflection, find the specific "drama" or proposal thread that
  prompted the note.
- For McKinsey space travel, choose actual companies. This one needs evidence
  more than vibes or it will turn into an empty rant.
- For the IM-1776 review, get the issue text rather than reviewing the landing
  page.
