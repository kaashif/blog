Java doesn't really get immutability
2022-10-23

This is a post from the perspective of a new Java programmer, so it is 100%
likely that the concerns here are well-known and already addressed. Or at
least discussed.

Java, as a language, doesn't get (understand) immutability and "delivers" it in
a way that grants almost none of the benefits of immutability in other
languages, like C++ or Rust. I picked those examples to show that the lesson
was learnt a long time ago (C++) and the lesson is still valid and a good idea
(Rust).

Java can, in some sense, be forgiven of its crimes because it's a pretty old
language and is stuck with backwards compatibility. But that doesn't mean it
doesn't commit those crimes.

The primary benefit of immutability is that the programmer knows that value
cannot be changed, so they no longer need to think about what would happen if
it did.

Java doesn't give you that and worse, it pretends that it does. Let's look at
some examples of Java lies and deception.

<!--more-->

## final

final is basically, from my perspective, useless. It protects against
reassignment, which isn't even nearly the most common type of mutability.

```java
final List<Integer> myList = new ArrayList<>();
myList.add(1);
myList.add(2);
```

I mean, no-one even pretends to think `final` is supposed to stop this, but
this kind of mutation is 99% of all mutation, and final doesn't stop it, so
what's the point of `final`?

There's also a rule that lambdas can't capture variables that aren't final or
effectively final because of possible race conditions. That's fine, but you can
capture a mutable object just fine, so that restriction seems to be completely
pointless.

An example continuing on from before. This isn't fine because i changes:

```java
int i = 1;
Supplier<Integer> badCapture = () -> myList.get(i);
i++;
```

Giving the error:

```
error: local variables referenced from a lambda expression must be final or effectively final
```

But this, despite being exactly the same thing, is fine:

```java
class MyInteger {
    int i = 1;

    public void incr() {
        i++;
    }
}

var i = new MyInteger();
Supplier<Integer> badCapture = () -> myList.get(i.i);
i.incr();
```

Now tell me, does that make any sense? If the rationale of banning captures of
non-final variables is that they might change, what's the rationale of allowing
captures of other variables that change in slightly different ways?

Whatever it is (and I'm sure there is one, somewhere), it adds to the list of
reasons to avoid Java. Either the language is so hobbled that true safety
measures *can't* be implemented, or everyone thinks this is fine.

## Immutable data structures

But (I hear you say) in my example I used an ArrayList, which is mutable! Why not
use an immutable list instead? Sure, let's give that a try:

```java
final List<Integer> myList = List.of();
myList.add(1);
myList.add(2);
```

This compiles, but when I run it:

```
Exception in thread "main" java.lang.UnsupportedOperationException
	at java.base/java.util.ImmutableCollections.uoe(ImmutableCollections.java:142)
	at java.base/java.util.ImmutableCollections$AbstractImmutableCollection.add(ImmutableCollections.java:147)
	at Main.main(example.java:9)
```

Oh, but I just used it wrong, right? NO! Java is wrong! This dreadful
"implementation" of immutable data structures violates the basic tenets of
programming. You can no longer treat a List as a List, because secretly it
might be immutable and fail at runtime!

This means if you're calling a library function:

```java
public int doACalculation(List<Integer> values);
```

You can't tell from the type signature whether it's safe to pass an immutable
list in, despite the fact that would implement the interface `List<Integer>`.
You haven't been saved by Java's immutable lists, they've just created
a whole different problem.

The name for the principle Java egregiously violates here is the Liskov
substitution principle:
<https://en.wikipedia.org/wiki/Liskov_substitution_principle>:

> It is based on the concept of "substitutability" - a principle in
> object-oriented programming stating that an object (such as a class) may be
> replaced by a sub-object (such as a class that extends the first class)
> without breaking the program.

Java breaks this because you can't replace a `List<T>` with any other `List<T>`
in all situations. If you mutate, you can only take mutable lists. If you
don't, you can take any list.

This is ridiculous because mutable operations are an obvious superset of
immutable operations. Immutable and mutable lists can both be read, but only
mutable ones can be written.

Mutable lists should be an extension of immutable lists!

Kotlin kind of tried to fix this by making immutable collections the default
(with an interface actually not supporting writing), and mutable ones an
extension of that, but the pain is still there.

Kotlin is still missing decent language level immutability controls (like C++'s
const), probably due to some bullshit (but entirely reasonable) concern about
Java interop.

## Conclusion

Please do not put up with the incredible farce that is Java immutablity. If at
all possible use some language that made a serious attempt to fix some of
Java's problems, like Kotlin or (if at all possible, again) something descended
from languages that already solved this problem, e.g. Rust. It's more likely
that you can move to Kotlin if you're using Java, so do that. It's really easy.

I can't say with any authority if Kotlin is really the best we can do to fix
the mistakes of Java, but it tries.

This isn't a very optimistic ending.
