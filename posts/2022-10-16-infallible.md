Why do programmers constantly and needlessly obfuscate code?
2022-10-16

People write code as if they think they'll always have the entire system in
their heads at all times, and thus consider hiding and obfuscating information
to be harmless. This is harmful for anyone reading the code in the future,
including (especially) the original author.

I can hear you saying that's ridiculous, and telling me to ask literally anyone
whether they think they have the entire codebase in their heads: they'll
definitely say no.

They may be lying - everyone will *say* fitting a huge codebase into their
mental working memory is impossible, but actions speak louder than words. Many
people (I see it all the time) constantly choose programming patterns and
idioms that only make sense if you think everyone coming after you will have
read, digested, and memorised all of the code.  There are a few really
important ones:

* Using nullability
* Using mutable state
* Using global (or static) variables

If you ever have to familiarise yourself with a codebase, or ever misremember
anything, you should try to encourage authors to avoid these as much as
possible. But people don't! They love to make things hard for reviewers and
future generations of code readers.

In the same breath as complaining about something a coworker has written,
people will go on to make the same faulty assumptions and obfuscate their code,
perhaps in a slightly different but materially equivalent way.

Let's look at why, and how to convince them to stop.
<!--more-->

## Nullability

This is a Java-focused blog post, but this equally well applies to any language
where things can be null - C++ has null pointers and `std::optional`, Python
has `None` that people like to pass around. Even Rust often has people passing
around `None` `std::option`s and going to great pains to handle that as
unsafely as possible.

It's well known that `null` was itself a mistake in Java. Here's some code that
a programmer committing the central fallacy might write:

```java
public Pet getPet() {
    if (!hasPet()) {
        // Means there's no pet
        return null;
    }

    if (hasDog()) {
        return getDog();
    }

    if (hasCat()) {
        return getCat();
    }

    // Means the pet type is unsupported by this function
    return null;
}
```

That seems reasonable. It even has comments. But the intention of this code is
*only* encoded in comments, not in the null values, which mean different
things. Suppose later the class is extended:

```java
public void adoptPet(Snake snake);
```

And a caller writes code expecting their snake pet to come back from `getPet`:

```java
petOwner.adoptPet(new Snake());
// ...
Pet pet = petOwner.getPet();
```

Obviously pet is null, but why? The original author of the getPet method assumed several things:

1. Future contributors would know what null meant and know where to look for
   the true meaning of null.

2. No-one would ever forget to update `getPet` to work for more than cats and dogs.

Solving (2) would be a different blog post entirely (sum types and enforced
exhaustive checking), but (1) has made diagnosing the problem unnecessarily
difficult - why is the value null? Comments don't appear in compiled Java
classes, if this quirk isn't in the PetOwner documentation, it may be
impossible to diagnose.

The solution is to avoid implicitly assigning meaning to null. You won't even
remember what you meant in 2 years when getPet gives you back null. If there's
an error, please please please don't just return null, throw an informative
exception.  And no - logging and returning null is not as good as an exception.

This applies in exactly the same way to `Optional.empty()`.

Generic nullability (e.g. null reference or Optional) is never good in my
opinion. Here are some alternatives:

* In Rust, prefer Result (which can include a message) over Option
* In Java, prefer exceptions over null and Optional
* In Haskell, prefer Either (which can include a message) over Maybe
* In Python, prefer exceptions over None

I hate generic nulls with a passion. If you see a null, it always means context
has been thrown away! Don't assume that future programmers will remember what
your null means or even be able to read your code to work it out.

A common theme in this post is that requiring callers to read through your
source code to be able to use your code is bad - they won't do it. The caller
should be violently and non-optionally confronted with relevant information
through type signatures or exceptions.

## Mutability

Java doesn't have support for immutability at the language level. This was in
my opinion a huge mistake - C++ had `const` when Java was being designed! Even
worse, some Java programmers believe that `final` is a substitute for
immutability.  It's not.

If an object is immutable, you only have to look at where the constructor was
called to know the state of the object.

If an object is mutable, you have to read all code referencing that object,
plus have full knowledge of the order that code is going to be called in. That
requires full knowledge of the codebase and gets impossible to manage after a
certain point.

Here's the interface of an object that relies on mutability:

```java
public class Executor {
    public void setParameter(String name, String value);
    public void execute();
}
```

The caller will set some parameters, then execute. Fine, how hard could that
be? Let's suppose this is the caller:

```java
public class FileManager {
    final Executor executor = new Executor();

    public void sendFile(File file) {
        executor.setParameter("filename", file.name());
        executor.execute();
    }

    public void deleteFile(File file) {
        executor.setParameter("deletion", "true");
        executor.execute(); 
    }
}
```

There's an obvious problem here - when we call deleteFile, we don't set the
filename. Is that fine? Maybe, if we always call sendFile for a given file
then we delete it right after.

The problem with the Executor is that it holds state, and that state changes.
When you're reading the code and you want to know what `executor` is, or what
will happen when you call `executor.execute()`, you don't just have to read
that line of code, you have to read *all lines of code that could possibly be
executed before it*.

(Note: final doesn't save us here and does almost nothing to help)

The solution here is to change the interface of Executor to avoid requiring
mutation:

```java
public class Executor {
    public void execute(Map<String, String> parameters);
}
```

Yes, you now have to provide a parameter map every time you call execute. But
this means that the outcome of execute should only depend on one line of code:
the line where you call execute.

This isn't exactly true because Java will still allow Executor to mutate
itself, and there's no way to stop that. In C++, you can stop that by making
execute const:

```c++
class Executor {
public:
    void execute(const std::map<std::string, std::string>&) const;
};
```

const gives us the guarantee that execute won't mutate its argument or itself.
That means we can free ourselves from needing to hold the entire codebase in
our minds just to work out what this one line does.

Rust has immutability by default, so this problem is solved by default there.
You can unsolve it by making things mutable if you want, but that's
discouraged.

Although Java doesn't allow you to make those kinds of guarantees at the
language level, teams who enforce immutability (except where needed e.g. for
high performance) will have an easier time understanding isolated lines of
code.

In particular, this is a godsend for pull request reviews - you no longer need
to build up a mental model of the temporal ordering of everything that happens
before every line of code. You can skip around and know no state is changing in
the lines you skip over, which is helpful since looking at diffs involves
skipping over many lines.

# Global variables

Singleton instances. Global variables. Static variables. These are all faces of
the same demon, which is already well known to be evil.  Testing is made
particularly hard when the global variables and implicit dependencies are
widespread, and that's the angle I'm coming at this from.

Suppose we have a database connection we want to share throughout our
application. You can get it through a static method:

```java
public class DatabaseConnection {
    public static DatabaseConnection getInstance();
}
```

And we have another class that already exists and has a lot of code:


```java

public class Server {
    public Server(OtherServiceClient client) {
        // ...
    }

    public void serveRequest() {
        // ...
    }

    public void cleanUp() {
        // ...
    }
}
```

Now let's suppose we write some unit tests:


```java
public class ServerTest {
    @Test
    void testServeRequest() {
        var server = new Server(mock(OtherServiceClient.class));

        // ...
    }
}
```

Has anything gone wrong here? Is this unit test isolated? We don't know *unless
we read the code under test*. There may or may not be a hidden, undeclared
dependency of Server on the database, and we'd need to mock that for our unit
tests.

In the worst case, a test author might write a unit test, happen to run it in
the right environment, and think they've isolated their test. In reality,
they're writing and cleaning out a real database somewhere.

I thought by this point everyone knew that sharing global state was bad, but in
Java land, somehow "Singleton" isn't a four-letter word like it should be.

If mutability is banned (which outlaws "setter injection" (which is utterly
disgusting)) and global variables are banned (which outlaws implicit
dependencies on shared state), the only route left is passing dependencies in
as part of the constructor. i.e.


```java
public class Server {
    public Server(OtherServiceClient client, DatabaseConnection connection) {
        // ...
    }
    // ...
}
```

There's no reason there can't still be a single DatabaseConnection, the only
change here is that we're explicitly declaring a dependency on
DatabaseConnection, and now there's no way a test author can accidentally
forget to mock a database, they must explicitly provide a database.

# Conclusion

I consider these truths self-evident and will broach no criticism. Mutability
hurts readability. Nulls hurt readability and usability. Globals hurt
readability and testability. The most effective way to avoid these is to use a
language where those problems have been solved - use languages that enforce
immutability, allow a wide range of meaningful non-null return values
(exceptions are fine, I guess), and never use global state!

Just say no to mutable state!
