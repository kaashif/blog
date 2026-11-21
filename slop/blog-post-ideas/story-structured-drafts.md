# Story-structured drafts

These are end-to-end story structures for every item in `blog-post-ideas.txt`.

Each entry has a start, an unchecked middle, and an ending. The middle is where
the real work belongs. Claims about commands, output, compile behavior,
benchmarks, documents, or runtime behavior are deliberately marked with:

`TODO: actually run this and check`

The aim is to avoid drafts that only explain the idea. Every post should begin
with a thing on the bench, do work in the middle, and finish with a narrow
conclusion that the work could actually support.

## 1. GCC extension for expression statements

Start: I write a C macro that needs local variables and still has to behave like
an expression. GNU C's `({ ... })` statement expression is the weird feature
that makes the macro possible.

Unchecked middle: TODO: actually run this and check: compile a `MAX(x++, y++)`
example with GCC, Clang in GNU mode, and strict C flags; inspect the warnings,
the expansion, and the argument evaluation count.

Ending: The post ends by saying that statement expressions are a practical way
to write safer macros in GNU C, but they also mark the point where the program
has stopped being portable C. The rule of thumb is simple: use them when the
build environment is already GNU-shaped, and do not smuggle them into code that
claims to be ordinary C.

## 2. MicroPython and CircuitPython

Start: I run the same tiny hardware program under MicroPython and CircuitPython
because "Python on a microcontroller" hides the details that actually matter.

Unchecked middle: TODO: actually run this and check: flash the same board with
both runtimes, run LED and sensor examples, compare filesystem behavior,
available modules, memory errors, and the edit-run loop.

Ending: The post ends with a concrete choice between the two. CircuitPython is
probably the better story if the workflow and supported boards matter most;
MicroPython may be the better story if control, ports, or language/runtime
details matter more. The conclusion should be tied to what happened on the
board, not to project branding.

## 3. Large files make Git slow

Start: I create a few repositories that abuse Git in different ways: one giant
blob, one giant blob changed repeatedly, and many small files.

Unchecked middle: TODO: actually run this and check: measure clone, checkout,
status, gc, repack, and repository size for plain Git and Git LFS.

Ending: The post ends by separating two claims. Git can store large files, but
its object model and history model make some large-file workflows painful. The
useful conclusion is a threshold-style rule: once large generated or binary
artifacts become part of normal history, use LFS or store them somewhere else.

## 4. `hyperfine` for profiling

Start: I time two tiny commands with `hyperfine`, then change the setup until
the winner changes.

Unchecked middle: TODO: actually run this and check: run warm and cold-cache
benchmarks, include and exclude shell startup, try parameter sweeps, and compare
with `time` and `perf stat`.

Ending: The post ends by treating `hyperfine` as a good measuring tape rather
than a proof machine. It is excellent for repeated command timing, but the user
still has to decide what is being measured. The last section should be a small
checklist for when `hyperfine` is enough and when a profiler is the next tool.

## 5. CWG 1331 and the `std::optional` mistake

Start: I write a tiny optional-like type that seems as though it should copy,
then hit the C++ rule behind CWG 1331.

Unchecked middle: TODO: actually run this and check: compile the smallest
example on current and older compilers, read the core issue, and compare the
observed behavior with the wording.

Ending: The post ends by saying that `optional` is simple at the API level and
annoyingly deep at the object-model level. The lesson is not that users need to
know every core issue, but that library authors are often fighting rules that
ordinary code never exposes directly.

## 6. C++ `std::error_code` and `<system_error>`

Start: I rewrite a small POSIX wrapper with `std::error_code` and compare it
with the usual `errno` version.

Unchecked middle: TODO: actually run this and check: wrap `open` and `read`,
create a custom error category, compare exceptions, `std::system_error`, and
`std::expected`.

Ending: The post ends with a guarded defence of `<system_error>`. It is verbose
because portable error domains really are verbose, but that does not make every
program better when it adopts the abstraction. The conclusion should say where
`std::error_code` buys clarity and where it only replaces one kind of boilerplate
with another.

## 7. GitHub Dependabot action for Maven auto-PRs

Start: I turn on Dependabot for a small Maven project and wait for it to open a
dependency update PR.

Unchecked middle: TODO: actually run this and check: configure Dependabot,
capture a generated PR, run CI, inspect the dependency tree before and after,
and include a transitive-dependency break if possible.

Ending: The post ends by refusing the simple advice to "turn it on and merge".
Dependabot is useful when paired with tests and a policy for which updates are
routine. Maven's dependency graph means the PR is the start of the review, not
the review itself.

## 8. `hardbool`

Start: I create a boolean-like integer type and then try to smuggle an invalid
bit pattern into it.

Unchecked middle: TODO: actually run this and check: find a compiler build or
patch with `hardbool`, compile examples, inspect diagnostics, and compare with
ordinary `_Bool`, enums, and sanitizer behavior.

Ending: The post ends by saying that `hardbool` is useful exactly where C's
"you know what I meant" attitude is dangerous: hardware, wire formats, and
corrupt inputs. It should also admit the limit: stricter booleans do not make C
a safe language, but they can make one common lie harder to tell.

## 9. Kafka is not a queue

Start: I build the same tiny work dispatcher with a normal queue and with Kafka.

Unchecked middle: TODO: actually run this and check: implement failures,
retries, consumer groups, replay, retention, and a second consumer that wants
the same history.

Ending: The post ends with Kafka described as a log that can support queue-like
workflows under specific conditions. The important distinction is deletion
versus retention, and acknowledgement versus offsets. Kafka is a good tool when
history and fanout matter; it is an awkward queue when those properties are
accidental.

## 10. TLA+ or something similar in Haskell or OCaml

Start: I model a small concurrent bug in TLA+ and then try to express the same
system in Haskell or OCaml.

Unchecked middle: TODO: actually run this and check: choose a lock service or
retry protocol, write the TLA+ spec, run the checker, then write an executable
functional model.

Ending: The post ends by saying which representation made the bug easiest to
find. The conclusion should be narrow: TLA+ is probably better at exploring
state spaces, while Haskell or OCaml may be nicer for executable structure. The
winner is the tool that exposed the mistaken assumption with the least ceremony.

## 11. V8 doubles on the heap vs CPython integers on the heap

Start: I allocate lots of numbers in JavaScript and Python and then look at the
runtime representation rather than the syntax.

Unchecked middle: TODO: actually run this and check: inspect V8 SMI and heap
number behavior, CPython `PyLongObject` layout, memory use, and array
specialization behavior.

Ending: The post ends by saying that "number" is a language-level idea, not a
storage layout. CPython and V8 make different bets about tagging, boxing, arrays,
and object identity. Performance follows those bets more than it follows the
surface fact that both languages are dynamic.

## 12. x64 assembler in Python with PeachPy

Start: I generate a small x86-64 function from Python using PeachPy.

Unchecked middle: TODO: actually run this and check: install PeachPy, generate a
function, assemble or load it, call it, and compare with handwritten assembly and
compiler output.

Ending: The post ends by deciding whether the Python layer removed real
repetition. If the example is a tiny add function, PeachPy looks like a party
trick. If the example involves SIMD variants or ABI-sensitive boilerplate, the
embedded assembler starts to make sense.

## 13. Write C to generate an ELF binary

Start: I write a C program whose output is an executable ELF file.

Unchecked middle: TODO: actually run this and check: generate a minimal x86-64
Linux ELF, run it, inspect it with `readelf` and `objdump`, and verify the exit
status with `strace` or the shell.

Ending: The post ends by making executable files feel less mystical. For the
smallest case, the loader needs a few headers, a mapped segment, and an entry
point. Sections, symbols, and linker convenience can come later; the first
working binary is mostly a contract with the kernel.

## 14. Locality of behaviour and spooky action at a distance

Start: I implement one small UI interaction in a local-behaviour style and in a
state-heavy style.

Unchecked middle: TODO: actually run this and check: build the same interaction
with htmx and with a more distributed frontend structure, then trace every file
or function involved in one click.

Ending: The post ends by saying whether the behavior was easier to inspect when
it lived near the markup. The conclusion should avoid framework theology. The
point is that local behavior makes some code easier to follow, while more
distributed state may be justified only when the interaction is complicated
enough to pay for it.

## 15. Folly `CoreCachedSharedPtr`

Start: I benchmark many threads repeatedly copying the same `shared_ptr`.

Unchecked middle: TODO: actually run this and check: build a Folly benchmark,
compare ordinary `shared_ptr`, `CoreCachedSharedPtr`, and a raw-pointer baseline
under increasing thread counts.

Ending: The post ends by showing that shared ownership has a physical cost when
many cores touch the same refcount. If Folly helps, it helps by moving or
reducing cache-line contention. The conclusion should say which access pattern
benefits, because this is unlikely to be a universal replacement for normal
`shared_ptr`.

## 16. Find a network driver to port to OpenBSD

Start: I pick a network card and read an existing driver from another operating
system next to a similar OpenBSD driver.

Unchecked middle: TODO: actually run this and check: choose the device, collect
docs, map probe, attach, DMA, interrupts, descriptor rings, and network-stack
handoff to OpenBSD equivalents.

Ending: The post ends even if the port fails. A failed port can still explain
where the translation became hard: missing docs, bus differences, DMA setup, or
network stack expectations. The result should be a map of the work, not a vague
claim that driver porting is hard.

## 17. Undo and time-travel debugging

Start: I debug a small program where the bad value is visible only after the
write that caused it is gone.

Unchecked middle: TODO: actually run this and check: record the program under
`rr` or Undo, reverse-continue to the bad write, and note which syscalls,
signals, and scheduling decisions are captured.

Ending: The post ends by saying that reversible debugging is powerful because it
turns "where did this come from?" into a navigable question. The limitation is
that replay requires controlling or recording the outside world. That cost is
why the tool feels magical in small cases and operationally serious in large
ones.

## 18. Haskell future monad vs Reactor Java

Start: I implement the same async fanout in Haskell and Reactor Java.

Unchecked middle: TODO: actually run this and check: include timeout,
cancellation, bounded concurrency, one failed call, and real stack traces or
logs from both versions.

Ending: The post ends by judging the unhappy path. The happy path is too easy in
both. The useful comparison is which version makes cancellation, failure, and
backpressure visible without filling the program with plumbing.

## 19. Java 21 ADTs from XSD `choice`

Start: I generate ordinary Java from an XSD `choice` and compare it with a Java
21 sealed-class representation.

Unchecked middle: TODO: actually run this and check: use a small nested
`choice`, run a normal generator, hand-write or generate sealed interfaces and
records, and compile pattern matching over the result.

Ending: The post ends by saying that XML schema `choice` is a sum type and Java
can now say that directly. The old nullable-field representation permits states
the schema forbids. Sealed classes do not make XML pleasant, but they move a
real invariant into the Java type system.

## 20. Heap after GC as a missing Micrometer feature

Start: I look at a JVM heap graph and want the number immediately after GC.

Unchecked middle: TODO: actually run this and check: build a Micrometer app,
listen for GC notifications, export heap-used-after-GC, and compare it with
ordinary heap-used during a leak or allocation-heavy workload.

Ending: The post ends by proposing the metric narrowly. Current heap usage is
useful, but post-GC heap is often a better live-set signal. Micrometer should
make that easy to expose, because every service reimplementing the same GC
listener is boring evidence that the abstraction is missing.

## 21. Pre-Java Oak

Start: I put early Oak material next to Java 1.0 and look at what survived.

Unchecked middle: TODO: actually run this and check: find primary Oak manuals,
source, or papers; quote short code samples; compare syntax, libraries, goals,
and the embedded-device story.

Ending: The post ends by saying which parts of Java look inherited from Oak and
which parts belong to the later public Java story. The useful conclusion is not
that Oak was secretly alien. It is that Java's beige public memory covers a
stranger product history.

## 22. Windows ABI vs Itanium/System V struct splitting

Start: I compile functions that pass and return small structs on Windows and on
a System V or Itanium-ABI-shaped target.

Unchecked middle: TODO: actually run this and check: inspect assembly for
`{int,int}`, `{int,double}`, arrays, and non-trivial C++ types; compare with ABI
documents.

Ending: The post ends by saying that "pass a struct" is not an ABI rule. The
real rules classify fields, registers, stack slots, hidden pointers, and
triviality. This is exactly why FFI code must match the platform ABI rather than
the programmer's intuition.

## 23. Futex

Start: I write a toy mutex using the Linux `futex` syscall.

Unchecked middle: TODO: actually run this and check: implement lock and unlock,
test with contended and uncontended threads, benchmark against `pthread_mutex`,
and inspect futex syscalls with tracing.

Ending: The post ends by saying that futexes are clever because the kernel does
not own the lock. User space owns the word and the fast path; the kernel owns
sleeping and waking on an address. That split is the useful idea, and most of
the complexity is paying for it without races.

## 24. Kotlin coroutines vs reactive for limiting concurrency

Start: I fetch many URLs with a maximum number of requests in flight, once with
Kotlin coroutines and once with a reactive style.

Unchecked middle: TODO: actually run this and check: implement both versions,
add timeouts and failures, measure peak concurrency, and inspect logs or stack
traces.

Ending: The post ends by saying where the concurrency limit is easiest to see.
Both styles can work. The better version for this problem is the one where
removing the limit looks obviously dangerous in code review.

## 25. Kryo serialization: how does it work?

Start: I serialize a Java object graph with Kryo and inspect the bytes.

Unchecked middle: TODO: actually run this and check: test registered and
unregistered classes, nulls, repeated references, cycles, and compare with Java
serialization and Jackson.

Ending: The post ends by explaining Kryo's speed through what it writes and what
it avoids writing. Registration, object identity, and reference tracking are not
library trivia; they are the serialization format. The conclusion should say
where the speed is bought with schema and compatibility discipline.

## 26. BetaArchive and Internet Explorer for Solaris

Start: I find an Internet Explorer for Solaris installer and try to run it.

Unchecked middle: TODO: actually run this and check: identify the exact Solaris
version and architecture, install or emulate the environment, capture installer
logs, screenshots, and dependency failures.

Ending: The post ends by using the installer and failures as historical
evidence. If it runs, the UI shows what Microsoft thought Unix browser users
needed. If it fails, the error messages show the world the software expected.
Either outcome is better than merely pointing at the weird product name.

## 27. Apache Tika for metadata extraction

Start: I feed Apache Tika a directory of PDFs, Office files, images, audio, and
broken files.

Unchecked middle: TODO: actually run this and check: compare Tika output with
`exiftool`, `pdfinfo`, and manual inspection; include corrupt and
password-protected files.

Ending: The post ends by treating Tika as useful but not omniscient. A universal
parser is valuable when you need broad coverage, but the wrong and missing
metadata are part of the contract. The conclusion should say how to use Tika
with enough suspicion.

## 28. Graal and Truffle: write a BASIC interpreter

Start: I write a tiny BASIC interpreter, then move it into Truffle.

Unchecked middle: TODO: actually run this and check: implement `PRINT`, `LET`,
`GOTO`, arithmetic, and maybe `FOR`; compare plain interpreter and Truffle-node
version under warmup.

Ending: The post ends by showing what Truffle needed in order to optimize. Graal
is not magic sprinkled on arbitrary code. The interpreter has to expose
structure, types, and stable execution patterns. The BASIC interpreter makes
that trade visible without requiring a serious language.

## 29. Winning strategy in QE

Start: I model a simplified game of QE, where players can bid any amount but
are punished for spending too much.

Unchecked middle: TODO: actually run this and check: read the rules, model
auction values and spending penalties, simulate bidding policies, and compare
with played games if possible.

Ending: The post ends with bidding rules that are explicitly limited. The point
is not to solve QE forever. The point is to turn the strange money rule into a
few numbers: when overbidding is rational, when fear of the penalty dominates,
and where the model stops matching a table of humans.

## 30. Do not use patching in Python

Start: I write a Python test that passes because `mock.patch` replaced the hard
part, then refactor the code and watch the test keep passing.

Unchecked middle: TODO: actually run this and check: build a tiny module with a
bad dependency boundary, test with `patch`, then rewrite with an explicit fake
or injected dependency.

Ending: The post ends with a narrow rule. Patching is sometimes the cheapest
tool for ugly code, but it should make you suspicious. If the test has to know
the import path rather than the program's dependency boundary, it may be testing
the wrong shape.

## 31. JSON vs Arrow serialization/deserialization speed

Start: I put the same dataset into JSON and Arrow, then measure the cost of
getting useful values back.

Unchecked middle: TODO: actually run this and check: benchmark parse, read,
write, filtering, memory use, Python bindings, and at least one compiled
language path.

Ending: The post ends with a boundary, not a scoreboard. Arrow should win when
typed columnar data and fewer copies matter. JSON should remain good when
inspectability, ubiquity, and loose structure matter. The useful answer is where
the conversion cost is paid and who pays it.

## 32. Atomics on x86 vs SPARC

Start: I compile the same C++ atomic examples for x86 and SPARC.

Unchecked middle: TODO: actually run this and check: inspect assembly for
relaxed, acquire, release, and sequentially consistent operations; run benchmarks
if hardware or emulation makes that meaningful.

Ending: The post ends by saying that C++ atomics are portable vocabulary over
non-identical machines. x86 can hide some costs with a strong model; SPARC may
make barriers and atomic instructions more visible. The conclusion should be
grounded in the emitted instructions.

## 33. Elixir and crashing as error handling

Start: I write an Elixir worker that crashes at random under a supervisor.

Unchecked middle: TODO: actually run this and check: capture supervisor logs,
restart behavior, retry behavior, and a stateful example where crashing halfway
through the operation matters.

Ending: The post ends by saying that "let it crash" is about failure boundaries,
not enthusiasm for exceptions. It works when the dying process owns state that
can be discarded or reconstructed. It does not remove the need to think about
files, payments, emails, and external side effects.

## 34. Annotation processor for compile-time XSD

Start: I write an annotation processor that reads an XSD and generates Java
types during compilation.

Unchecked middle: TODO: actually run this and check: generate Java 21 records
and sealed classes from a small schema, test compiler errors, incremental
builds, and IDE behavior.

Ending: The post ends by deciding whether compile-time schema handling improves
the developer experience. If the generated code is readable and errors point
back to the schema, it is useful. If failures become obscure annotation
processor noise, the cure may be worse than runtime parsing.

## 35. Transcript search with Rust and embedded SQLite

Start: I rebuild transcript search as a Rust program over SQLite FTS.

Unchecked middle: TODO: actually run this and check: import a transcript corpus,
measure index size, indexing time, query latency, ranking quality, and compare
with the older setup.

Ending: The post ends by saying whether "one binary and one SQLite file" is
enough for this job. If it is, the operational simplicity is the main result. If
ranking or updates are bad, the post should say what PostgreSQL or the old stack
was buying.

## 36. Shell scripts as an escape hatch out of unit tests

Start: I take a small shell script that has become load-bearing and try to test
it.

Unchecked middle: TODO: actually run this and check: add `shellcheck`, add a
small `bats` test, break quoting or error handling intentionally, and compare
with moving logic into a normal language.

Ending: The post ends by being fair to shell. It is excellent glue and a bad
place to hide business logic. The practical conclusion is to keep orchestration
in shell, move decisions and data manipulation elsewhere, and test the boundary
where shell still remains.

## 37. Compression algorithm implementation

Start: I implement a tiny compressor and decompressor for one simple scheme.

Unchecked middle: TODO: actually run this and check: choose Huffman, LZ77, or
LZSS; test text, repeated bytes, binary data, and random data; inspect the
encoded output.

Ending: The post ends by saying that compression is a bet about input shape.
The toy compressor should work on data with obvious redundancy and fail or grow
on random data. That failure is part of the explanation, because real formats
exist to handle the cases the toy version avoids.

## 38. Performant rounding

Start: I write several rounding functions and test them on values that usually
break casual rounding code.

Unchecked middle: TODO: actually run this and check: test negative values,
halfway cases, NaNs, overflow, standard functions, CPU instructions, and custom
batch rounding.

Ending: The post ends by tying speed to correctness. Adding `0.5` and truncating
is appealing because it is short, but the correct operation has a specification.
The conclusion should say which implementation is fast enough after it has
survived the edge cases.

## 39. Rust `bumpalo`

Start: I build a tree or parser structure first with ordinary allocation and
then with `bumpalo`.

Unchecked middle: TODO: actually run this and check: measure allocation count,
runtime, memory use, and the lifetime errors that appear when arena-owned values
try to escape.

Ending: The post ends by saying that arena allocation is fast because the
lifetime story is simple. Rust makes that story explicit. If all the nodes die
together, `bumpalo` is a good fit; if values need independent lifetimes, the
allocator is fighting the program.

## 40. Rust audio synthesizer latency

Start: I write a Rust synth that produces a sine wave and then deliberately make
the audio thread do suspicious things.

Unchecked middle: TODO: actually run this and check: use `cpal` or similar,
measure buffer size and latency, add allocation, logging, locking, and
cross-thread control messages.

Ending: The post ends by saying that real-time audio is ordinary programming
under a deadline. The code that is fine in a web request can click, underrun, or
lag in an audio callback. The useful rules are the boring ones: avoid allocation
and locks on the callback, keep work bounded, and measure the buffer behavior.

## 41. Write a gRPC service

Start: I write a small gRPC service with one unary method and one streaming
method.

Unchecked middle: TODO: actually run this and check: inspect the `.proto`,
generated code, deadlines, cancellation, status errors, and HTTP/2 behavior if
practical.

Ending: The post ends by saying what gRPC buys for the example. Typed generated
clients and streaming may be worth the machinery. For a tiny JSON endpoint, they
may not be. The conclusion should be based on the service that was built, not on
general claims about RPC.

## 42. How Java objects are laid out in memory

Start: I make a Java class with one `int` field and use JOL to inspect its real
size.

Unchecked middle: TODO: actually run this and check: run JOL on small classes,
arrays, strings, inheritance examples, compressed oops, and different alignment
settings.

Ending: The post ends by saying that Java object cost is not the sum of field
sizes. Headers, alignment, references, and arrays matter. The practical lesson
is that millions of tiny objects are a memory-layout decision, even when the
source code looks harmless.

## 43. GPTs to make a biography of your parents

Start: I record a family-history interview and try to turn the transcript into a
structured biography draft.

Unchecked middle: TODO: actually run this and check: design questions, get
consent, transcribe, extract chronology, generate a draft, and mark every claim
requiring human verification.

Ending: The post ends by setting a hard boundary around invention. The useful
tool is a clerk: organize stories, find gaps, suggest follow-up questions, and
produce editable structure. It must not invent dates, motives, or emotional
closure for people who are not there to correct it.

## 44. Bubblewrap containers

Start: I use `bubblewrap` to run a shell with a deliberately strange filesystem
view.

Unchecked middle: TODO: actually run this and check: build the sandbox step by
step with bind mounts, tmpfs, read-only paths, optional network isolation, and a
comparison with Docker.

Ending: The post ends by making containers feel like composed kernel features.
`bubblewrap` is useful because it exposes namespaces and mounts without a large
container product around them. The conclusion should also state what it does not
provide, especially around images, orchestration, and policy.

## 45. Polonius borrow checker

Start: I find a Rust program that looks safe to a human but is rejected by the
current borrow checker.

Unchecked middle: TODO: actually run this and check: test canonical Polonius
examples on stable and nightly Rust, enable available flags, and read the loan
analysis explanation.

Ending: The post ends by saying what fact the current checker loses and how
Polonius represents it differently. If the example still fails on today's
compiler, that is part of the result. The point is precision, not a complaint
that Rust should accept every program that looks plausible.

## 46. `Libfn/functional`

Start: I rewrite a small C++ pipeline with `optional` or `expected` using
`Libfn/functional`.

Unchecked middle: TODO: actually run this and check: identify the exact repo,
build it, rewrite the pipeline, compare with plain C++23, and inspect compiler
errors.

Ending: The post ends by deciding whether the library removes control-flow noise
or only changes its costume. C++ functional style is worthwhile when the program
gets easier to read. If it mainly produces template-heavy cleverness, the
ordinary `if` statements win.

## 47. `uv` being much faster than `pip`

Start: I install the same Python projects with `pip` and `uv` and break the time
down by phase.

Unchecked middle: TODO: actually run this and check: benchmark fresh installs,
cached installs, resolver-only cases, lockfiles, ugly dependency graphs, and
wheel-building cases.

Ending: The post ends by saying where `uv` is faster and where the bottleneck
moves elsewhere. If resolving and installing become cheap, Python development
feels different. If network or building dominates, the tool cannot remove that
cost. The conclusion should be about workflow, not benchmark theatre.

## 48. Rust destructors are not guaranteed

Start: I make a Rust type with `Drop`, then make the destructor fail to run
without using `unsafe`.

Unchecked middle: TODO: actually run this and check: demonstrate `mem::forget`,
`Rc` cycles, panic abort, process exit, and ordinary drop order.

Ending: The post ends by saying that Rust gives deterministic destruction in
normal ownership flow, not metaphysical finalization. Leaking is memory-safe
even when it is resource-bad. Unsafe code and resource wrappers must be written
with that distinction in mind.

## 49. Rust static reflection drama vs C++ reflection

Start: I solve the same compile-time field-introspection task in Rust and in
current or proposed C++.

Unchecked middle: TODO: actually run this and check: identify the Rust
discussion, implement with derives or proc macros, compare with C++ reflection
proposals or experimental compilers.

Ending: The post ends by separating the technical need from the community
argument. Both languages want field information at compile time. Rust's answer
often goes through proc macros; C++'s answer goes through proposals shaped by
template history. The conclusion should say which approach makes the simple task
less unpleasant.

## 50. Codon as faster Python with a JIT/compiler

Start: I run a few Python programs through Codon and compare them with CPython.

Unchecked middle: TODO: actually run this and check: benchmark numeric loops,
object-heavy code, imports, unsupported features, CPython, PyPy, and Codon.

Ending: The post ends by drawing the compatibility boundary. Codon may be very
good for Python-shaped numeric code while still being a different practical
environment from ordinary Python. The useful conclusion is which programs move
cleanly and which ones reveal the difference.

## 51. Hazard pointers

Start: I write or inspect a lock-free stack and run into the problem of freeing
removed nodes safely.

Unchecked middle: TODO: actually run this and check: demonstrate unsafe
reclamation, add hazard pointers, compare with epoch reclamation, and show a
two-thread interleaving.

Ending: The post ends by saying that memory reclamation is often the real
lock-free problem. Atomically removing a node is only half the job. Hazard
pointers solve the other half by making each reader publish what it might still
touch.

## 52. Varargs C++ functions cannot be coroutines

Start: I try to put `co_await` inside a C++ function with C-style varargs and
collect the compiler rejection.

Unchecked middle: TODO: actually run this and check: compile varargs coroutine
examples with GCC and Clang, compare with variadic templates, and inspect the
coroutine-frame explanation.

Ending: The post ends by saying the restriction is less arbitrary once coroutine
frames are visible. The compiler has to save parameters and locals across
suspension. Typed variadic templates and C ellipses give it very different
amounts of information.

## 53. Rewinding C++ coroutines with memory trickery

Start: I inspect a coroutine frame and try to mutate the saved state before
resuming it.

Unchecked middle: TODO: actually run this and check: find the state slot for one
compiler and optimization level, change it, observe behavior, then repeat on
other builds.

Ending: The post ends by treating the result as archaeology, not advice. If the
state jump works, it teaches something about one lowering. If it crashes or
changes under optimization, that teaches the more important lesson: the frame is
not a portable interface.

## 54. Python ten million threads: no-GIL vs async

Start: I compare many sleeping Python threads with many async tasks under a
current free-threaded or no-GIL build.

Unchecked middle: TODO: actually run this and check: measure memory, startup
time, scheduling, wakeup cost, CPU-heavy work, and I/O-heavy work.

Ending: The post ends by separating concurrency from parallelism. No-GIL changes
CPU parallelism for Python code, but it does not make OS threads free. Async
remains a compact representation for huge numbers of waiting operations. The
measurements should show where each model runs out of road.

## 55. `sysret_ss_attrs` AMD64 design mistake

Start: I find the kernel workaround or comment for the AMD64 `SYSRET` segment
state issue.

Unchecked middle: TODO: actually run this and check: identify the exact erratum
or bug, trace syscall return paths, read Linux or BSD code, and draw the state
transition.

Ending: The post ends by saying that long mode did not free operating systems
from every old segmentation detail. A small architectural decision can become
permanent kernel workaround code. The conclusion should be grounded in the
actual return path, not in general CPU folklore.

## 56. Linux `readv` and `writev`

Start: I write a header-plus-body output example first with concatenation and
then with `writev`.

Unchecked middle: TODO: actually run this and check: test files and sockets,
partial writes, syscall counts, buffer lifetimes, and performance against manual
copying.

Ending: The post ends by saying that scatter/gather I/O is useful when the
program already has pieces. It avoids a copy and a temporary buffer, but it does
not remove partial writes or lifetime rules. The correct loop is the important
example.

## 57. Tearing of 64-bit Java types

Start: I try to catch a torn `long` read and compare the experiment with the
Java memory model wording.

Unchecked middle: TODO: actually run this and check: read current JLS text, test
plain, `volatile`, and atomic long access on available JVMs, and search for old
32-bit behavior.

Ending: The post ends by separating spec allowance from common modern behavior.
If the experiment never tears, the spec history is still interesting. The lesson
is that language memory models carry old machine constraints longer than most
programmers notice.

## 58. Is Tokio WASM-compatible?

Start: I compile small Tokio examples to WebAssembly and record what works.

Unchecked middle: TODO: actually run this and check: test browser WASM and WASI,
timers, spawning, TCP, files, threads, and feature flags.

Ending: The post ends with a compatibility table and a reason for it. Tokio is a
runtime over assumptions about its environment. Some async futures are portable;
OS-shaped features need an OS-shaped WASM environment or a replacement API.

## 59. Is pip's dependency solver Turing-complete?

Start: I try to encode a small hard problem using Python package versions and
constraints.

Unchecked middle: TODO: actually run this and check: study pip's resolver,
encode SAT-like instances, measure resolver behavior, and be precise about
decidable search versus Turing completeness.

Ending: The post ends by correcting the slogan if needed. Dependency solving
can be finite and still ugly. If "Turing-complete" is false, the better
conclusion is the true complexity claim and a small evil dependency graph that
makes the pain visible.

## 60. `rkyv` for zero-copy deserialization

Start: I archive a Rust struct with `rkyv` and read it back without constructing
a fresh owned object graph.

Unchecked middle: TODO: actually run this and check: inspect archived bytes,
benchmark against `serde_json`, `bincode`, and postcard, and test validation,
alignment, endianness, and versioning.

Ending: The post ends by saying that zero-copy is a layout bargain. It can be
very fast when bytes are already in the shape the program wants. That speed is
paid for with rules about layout, validation, and compatibility.

## 61. Rust profiling: `hyperfine` and Codespeed

Start: I benchmark a Rust CLI locally and then set up or sketch continuous
performance tracking.

Unchecked middle: TODO: actually run this and check: run `hyperfine`, add a
microbenchmark where useful, configure a Codespeed-style dashboard, and measure
noise.

Ending: The post ends by placing tools on a ladder. `hyperfine` is good for a
local question; continuous benchmarking is for noticing regressions after the
local context is gone. The practical conclusion is the smallest setup that
catches real changes without drowning in noise.

## 62. Python `-O` optimization/debug flag

Start: I run the same Python script normally and with `python -O`.

Unchecked middle: TODO: actually run this and check: inspect `assert`,
`__debug__`, `-OO` docstring behavior, bytecode differences, and any measurable
speed changes.

Ending: The post ends by saying that `-O` is mostly a semantic switch around
debug behavior, not a magic speed button. Assertions are for internal sanity
checks that may disappear. Validation that must run in production belongs in
ordinary code.

## 63. ARM Jazelle: JVM bytecode on the CPU

Start: I read public Jazelle documentation and try to determine what hardware
actually executed.

Unchecked middle: TODO: actually run this and check: find manuals, device
references, emulator support, bytecode handling details, and what trapped back
to software.

Ending: The post ends by putting Jazelle in its mobile-Java context. Hardware
bytecode support sounds strange now, but it was a plausible response to Java ME
and constrained devices. The conclusion should say what the hardware really did,
and why the idea faded.

## 64. `sockaddr` fake flexible array

Start: I follow a `bind` or `connect` call through `sockaddr`,
`sockaddr_in`, and `sockaddr_storage`.

Unchecked middle: TODO: actually run this and check: print sizes and offsets on
several systems, inspect headers, and trace how the length argument controls the
real address shape.

Ending: The post ends by saying that the socket API is an ABI convention dressed
as a type hierarchy. The cast works because the length and family fields carry
the truth. `sockaddr_storage` is the later, larger box built around the same old
pattern.

## 65. SJLJ exceptions and C++ exception performance

Start: I compile exception examples using different exception mechanisms or
targets.

Unchecked middle: TODO: actually run this and check: find SJLJ support, compare
throw and non-throw paths, inspect assembly, code size, and unwind tables.

Ending: The post ends by clarifying "zero-cost". Table-based exceptions try to
keep the normal path cheap, while throwing still costs real work. SJLJ pays in
different places. Exception-performance claims should always say which path is
being measured.

## 66. gRPC request hedging benchmarks

Start: I build a fake service with a configurable latency distribution and put a
hedging client in front of it.

Unchecked middle: TODO: actually run this and check: vary hedge delay, load,
latency distribution, cancellation timing, p50, p95, p99, extra requests, and
wasted work.

Ending: The post ends by pricing the tail-latency improvement. Hedging can buy
better high percentiles by spending spare capacity. When spare capacity is gone,
the same trick can amplify load. The conclusion should be a graph-shaped rule
for when the purchase is worth it.

## 67. Conservative GC: do not keep track of references

Start: I run a small C program under a conservative garbage collector and make a
pointer-looking value keep memory alive.

Unchecked middle: TODO: actually run this and check: use Boehm GC, construct
real pointers and false positives, and inspect which allocations survive.

Ending: The post ends by saying that conservative GC is a compromise that makes
C possible to collect. It works without exact pointer maps, but false positives
and non-moving behavior are consequences of that choice. The compromise is the
feature.

## 68. `std::inplace_vector` and L1 cache

Start: I replace a small `std::vector` with inline-capacity storage and measure
the difference.

Unchecked middle: TODO: actually run this and check: use available
`inplace_vector`, benchmark small sizes, inspect allocation count, object size,
copy cost, and cache behavior.

Ending: The post ends by saying that inline storage is good when small size is a
real invariant. It avoids heap allocation and can improve locality, but it also
makes the containing object larger. Fixed capacity is an optimization only when
the program actually has a fixed-small shape.

## 69. Double-wide atomic instructions: are they good yet?

Start: I check whether a 128-bit atomic compare-and-swap is lock-free on the
machines and compilers in front of me.

Unchecked middle: TODO: actually run this and check: test platform intrinsics,
`std::atomic<__int128>`, alignment requirements, generated code, and tagged
pointer examples.

Ending: The post ends by separating existence from usability. A double-wide
atomic that compiles by calling a lock is not the primitive lock-free code was
hoping for. The practical conclusion is platform-specific: whether it is boring
enough to depend on.

## 70. AVX-512 Galois-field instructions

Start: I write a small GF(2^8) operation using AVX-512 GFNI instructions.

Unchecked middle: TODO: actually run this and check: find suitable hardware,
implement scalar, table-based, and GFNI versions, then benchmark and inspect the
actual instructions.

Ending: The post ends by making the instruction less ridiculous without making
it ordinary. GFNI is specialized, but finite-field byte operations appear in
real crypto, checksums, and erasure-coding work. The conclusion should say what
the instruction is good at and why most programs will never care.

## 71. McKinsey space travel

Start: I collect specific frontier-market startups with former consulting or
elite MBA operator backgrounds and read their public claims.

Unchecked middle: TODO: actually run this and check: verify founders, funding,
product claims, hiring pages, technical material, customers, and whether any
space-travel example actually fits the pattern.

Ending: The post ends only if the evidence supports the pattern. Some companies
may be real technical businesses with polished operators. Others may be
consulting language pointed at hard tech. The conclusion should name the
difference rather than sneer at credentials.

## 72. Review of IM-1776 issue on health

Start: I read IM-1776 Issue VI, "Health of a Nation", as a whole issue rather
than as a proxy for a political tribe.

Unchecked middle: TODO: actually run this and check: read every essay, extract
testable claims, check factual claims, and separate argument, mood, and
aesthetic posture.

Ending: The review ends by saying which essays earned their claims and which
ones borrowed authority from mood. A health issue can be literary and political,
but factual claims still need evidence. The conclusion should judge the issue by
what it actually prints.

## 73. `nanobind` for Python bindings

Start: I bind the same small C++ API with pybind11 and `nanobind`.

Unchecked middle: TODO: actually run this and check: compare source size, build
setup, binary size, import time, call overhead, NumPy support, and lifetime
behavior.

Ending: The post ends by saying whether `nanobind` improves this binding task.
The right comparison is not hello world. It is ownership, arrays, exceptions,
and build friction. If it is better, the diff should show where.

## 74. Python performance tips, loops, calls, CPython vs Codon

Start: I turn common Python performance tips into benchmarks.

Unchecked middle: TODO: actually run this and check: measure function calls,
loops, comprehensions, builtins, vectorized libraries, CPython, PyPy, and Codon.

Ending: The post ends by demoting slogans to conditions. Function-call overhead
matters when almost nothing else happens. Python loops matter when the work
stays in Python. A tip is useful only when it says which workload it describes.

## 75. Python GIL can release in the middle of `+=`

Start: I use `+=` on a custom object and inspect the bytecode and method calls.

Unchecked middle: TODO: actually run this and check: test `__iadd__`, `__add__`,
allocation behavior, C extension calls, and known GIL-release points.

Ending: The post ends by saying that the GIL is not a transaction system.
Specific operations may be atomic in specific CPython builds, but arbitrary
Python expressions can call user code and do surprising work. Correct concurrent
code needs a clearer guarantee than folklore.

## 76. Java JIT deoptimization when a feature flag flips

Start: I write a Java loop controlled by a feature flag and watch JIT
compilation and deoptimization.

Unchecked middle: TODO: actually run this and check: enable JIT logs, use
JITWatch, flip the flag at runtime, and record assumptions, compilation, and
deoptimization events.

Ending: The post ends by saying that deoptimization is the safety valve that
makes speculative optimization possible. The JVM is allowed to optimize for the
world it has seen, provided it can retreat when the world changes. A feature
flag is a boring business object that exposes that mechanism nicely.

## 77. CPython GC cycle detection

Start: I create two Python objects that reference each other, delete the outside
names, and inspect why refcounting alone cannot free them.

Unchecked middle: TODO: actually run this and check: use `gc` introspection,
tracked and untracked objects, `__del__`, collection stats, and CPython source.

Ending: The post ends by saying that CPython is refcounted until refcounting
cannot answer the question. Cycles require a separate collector that finds
groups reachable from themselves but not from the program. That extra collector
is not an implementation detail users never feel; it explains finalizer
weirdness, pauses, and GC controls.

## 78. CPython GC: why not just copy Java?

Start: I ask why CPython cannot simply use a JVM-style moving generational
collector.

Unchecked middle: TODO: actually run this and check: read CPython C API
constraints, borrowed references, object address assumptions, extension-module
behavior, and compare with JVM safepoints.

Ending: The post ends by saying that runtime design is path dependence made
executable. The JVM can move objects because its world is built around managed
references and safepoints. CPython exposes too much object reality to C code for
that swap to be simple. Copying Java would mean changing Python's compatibility
contract.

## 79. C++ memory order

Start: I write tiny atomic examples and compile them for different targets.

Unchecked middle: TODO: actually run this and check: test counters,
message-passing, spinlocks, double-checked initialization, x86 assembly, ARM
assembly, and any observable runtime behavior.

Ending: The post ends with boring rules. Use relaxed when there is no ordering
story. Use acquire and release when publishing data. Reach for `seq_cst` when
the stronger global order is worth the cost or when the proof is otherwise too
fragile. Every example should justify the order it uses.

## 80. Java scoped values vs Python context variables

Start: I implement request-context propagation in Java and Python and follow a
request ID across calls, threads, and async boundaries.

Unchecked middle: TODO: actually run this and check: test Java scoped values,
virtual threads, Python `contextvars`, asyncio tasks, normal threads, callbacks,
and propagation failures.

Ending: The post ends by saying that dynamic context is useful when explicit
parameters become tedious and dangerous when hidden inputs become invisible. Java
and Python draw different propagation boundaries. The useful comparison is where
the request ID appears, where it vanishes, and whether that behavior is obvious
from the code.

