Decomposing representations: a journey into computational group theory
2020-04-16


For my Master's degree, I (helped greatly by my supervisor)
implemented some algorithms and even invented some new algorithms to
decompose representations of finite groups. I wrote an extremely long
(well, relative to other things I've written) and technical thesis
about this, but I find myself increasingly unable to understand what
any of it means or why I even have a degree.

I thought being forced into a short-form blog post would help me
remember whatever it is I spent a few years studying to do. There are
some basic questions:

* What is a group?
* What is a representation?
* What is a decomposition of a representation?

And some interesting questions, with some computational tricks
relevant to a wider audience:

* Why is this useful?
* How do you get a computer to do it?
* How do you get a computer to do it, quickly?

These are the questions I'll attempt to answer in this blog
post. It'll be fun!
<!--more-->

## What is a group?

This is an easy one:
[read the Wikipedia page](https://en.wikipedia.org/wiki/Group_(mathematics)). In
short: a group is:

* A set $G$ (for example, the real numbers or the rational numbers)

* With an binary operation $\ast$ (like addition or multiplication,
  takes two elements of $G$ and gives you another element of $G$)

* Such that $\ast$ is associative (bracketing doesn't matter - $(a
  \ast b) \ast c = a \ast (b \ast c)$, like multiplication or
  addition, but NOT like subtraction)

* Such that $G$ has an identity element with respect to $\ast$ (an
  element $e$ such that applying it with $\ast$ does nothing, like
  multiplying by 1 or adding 0).

* Such that elements of $G$ all have inverses with respect to
  $\ast$ (any element $g \in G$ has another element $g^{-1} \in
  G$ such that combining them with $\ast$ gives the identity, like
  any number gives 0 when added to its negative, or 1 when multiplied
  with its reciprocal)

Examples include the real numbers $\mathbb{R}$ with addition and the
non-zero rational numbers $\mathbb{Q}^\ast$ with multiplication. There
are also finite examples, like the group of invertible maps from an
$n$ element set to itself, with composition of maps as the operation,
this is called $S_n$ (the symmetric group on $n$ elements), which has
size $n!$.

## What is a representation?

A representation $\rho : G \to \text{GL}(V)$ of a group $G$, is a
homomorphism (a function respecting the group structure) from $G$ to
the group of linear automorphisms of a vector space $V$. You can
imagine "vector space" to mean $\mathbb{C}^n$ (the space of length $n$
vectors with entries in the complex numbers), since that's usually
what we're using.

"Linear automorphism" is a coordinate-independent way of saying
"invertible matrix". Sue me if you don't like it, but you'll never get
your vector space into a computer without picking a basis.

## What is a decomposition of a representation?

An irreducible representation is a representation that doesn't have
any subrepresentations. What does that mean? We could go through the
definitions, but for complex representations of finite groups,
irreducible means indecomposable: you can't write the representation
as a direct sum of other representations. That is the same thing as
saying you can't simultaneously block diagonalise all $\rho(g)$ for $g
\in G$ such that the blocks are smaller than the whole matrix.

Now that the basics are out of the way, we can ask an interesting
question:

## Why is this useful?

My time at university was spent studying inaccessible, highly
theoretical examples. The primary application of my work, which I
wrote a good 30 pages about, isn't actually very... *applied*.

That said, irreducible representations come up in all sorts of places:

* Solving problems in quantum chemistry involving symmetries (oh,
  hello $S_n$)

* Solving completely computationally intractable optimization problems
  by spotting interesting symmetries ($S_n$), then reducing them

* Turning groups into linear algebra, then studying them through how
  their representations behave.

But most of the time, physicists are interested in infinite groups and
in particular, Lie groups. You might have heard of the Lorentz group
$\text{O}(1,3)$, a certain special unitary group $\text{SU}(2)$, or
some other Lie groups - these pop up all the time in physics.

Sadly, these algorithms are not directly applicable to those cases,
since they're too continuous to fit inside my poor computer.

## How do you get a computer to compute a decomposition?

The first question to ask is how do you do it?

The best way to come at anything related to representations is via
character theory. The character $\chi$ of a complex representation
$\rho$ is the map $g \to \text{trace}(\rho(g))$. This has some very
useful properties.
