---
layout: post
title: "One simple trick to improve your data classes (bugs hate it!)"
subtitle: "Static factory methods + Optionals = quality data"
permalink: "/blog/improving-data-classes"
---

#### *Warning: Java ahead*

The humble data class: as commonplace in enterprise software as a cardboard box in a
warehouse. Data classes are usually barely more than type-safe dictionaries for naming
bundles of related data, and we find them so tedious to write that, in a language like
Java, we've created tools like [Lombok](https://projectlombok.org) to automate the
generation of most of their boilerplate code for us. And yet, also like cardboard boxes
in a warehouse, data classes play a central role within the larger system. So why not
ensure that they're robust and safe to handle?

### The status quo

I like to play a lot of board games. Since tracking the minutiae of our lives is in
vogue these days, it seems apropos that I build an application to help me keep track of
every time I play a board game.

A reasonable first step might be to construct the central `Playthrough` data class for
this application. Starting out, it may look something like this[^1] (in vanilla Java):

```java
public final class Playthrough {
  private final LocalDate date;
  private final Game game;

  public Playthrough(LocalDate date, Game game) {
    this.date = date;
    this.game = game;
  }

  public LocalDate getDate() {
    return this.date;
  }

  public Game getGame() {
    return this.game;
  }
}
```

This template for defining a data class is certainly usable, but following the common
practice of using a public all-arguments constructor to create new instances arguably
leaves it with an overly permissive contract. This is especially true of a language that
allows null references. Case in point, using this implementation, it is possible to
construct the following three `Playthrough` instances:

- ```java
  Playthrough badDate = new Playthrough(null, Game.SPLENDOR);
  ```
  {: .language-descriptor-hidden}

- ```java
  Playthrough badGame = new Playthrough(LocalDate.of(2022, 2, 3), null);
  ```
  {: .language-descriptor-hidden}

- ```java
  Playthrough badDateAndGame = new Playthrough(null, null);
  ```
  {: .language-descriptor-hidden}

The fact that these instances could theoretically show up during runtime is unfortunate,
both in theory because they fail to model anything meaningful, and in practice because
they often lead to runtime errors.

Take the following function, for example, which computes the game I played most
recently:

```java
/**
 * Determines the most recently played game from a collection of playthroughs
 *
 * <p>Returns an empty Optional if the given collection is empty
 */
public static Optional<Game> determineMostRecent(
    Collection<Playthrough> playthroughs) {
  return playthroughs.stream()
      .max(Comparator.comparing(Playthrough::getDate))
      .map(Playthrough::getGame);
}
```

In spite of its rather modest appearance, this function is technically not safe to call
without either wrapping the call inside a try-catch block to handle a potential null
pointer exception or, preferably, ensuring that none of the `Playthrough`s in the
provided collection have empty dates before making the call.

More generally, the phrase "garbage in, garbage out" applies well to situations like
this. When we define our data classes in such a way that they can store bad data
("garbage in"), we in turn must must either introduce additional complexity into our
code to compensate for this, or else run the risk of introducing bugs into our software
("garbage out"). Wouldn't it be nice if we could avoid all this with only a few small
tweaks?

### A first-pass improvement

Just like how a `Playthrough` without a date doesn't make sense, we are hard-pressed to
find a reasonable interpretation of the following `LocalDate` instance:

```java
LocalDate badDay = LocalDate.of(2022, 2, 0);
```
{: .language-descriptor-hidden}

Despite its meaninglessness, this code compiles without issue, just like the previously
listed nonsensical `Playthrough` examples. The key difference, however, is that
attempting to actually instantiate this date at runtime raises a rather informative
error:

```text
java.time.DateTimeException: Invalid value for DayOfMonth (valid values 1 - 28/31): 0
```
{: .language-descriptor-hidden}

Attempting something similar with the month field results in an analogous error. As a
result, any `LocalDate` that is successfully created with  [the
`LocalDate#of(int, int, int)` factory
method](https://docs.oracle.com/javase/8/docs/api/java/time/LocalDate.html#of-int-int-int-)
must *necessarily* contain valid data, because otherwise an exception would have been
thrown during initialization. Assuming that similar precautions were taken for all
`LocalDate` factory methods, as a user of the `LocalDate` class I can rest easy knowing
that any instance I come across represents a valid, meaningful date.

In my opinion, this is a marked improvement on any approach that permits storing bad
data, since it eliminates the possibility of said bad data causing an unexpected error
at any point after initialization. However, the use of an unchecked exception still
leaves open the possibility of an unhandled error escaping *during* initialization, so
the danger posed by bad data is not entirely eliminated. Fortunately, simply replacing
an unchecked exception with a checked one improves the situation even further by forcing
users to explicitly handle the issue of providing bad data during initialization. Taking
all this into account, I might update my `Playthrough` class like this:

```java
public final class Playthrough {
  public static final class InvalidDataException extends Exception {}

  private final LocalDate date;
  private final Game game;

  /** @throws InvalidDataException if the initialization data is invalid */
  public Playthrough(LocalDate date, Game game) throws InvalidDataException {
    if (!isValid(date, game)) throw new InvalidDataException();
    this.date = date;
    this.game = game;
  }

  /** The given data is valid if and only if both date and game are not null */
  private static boolean isValid(LocalDate date, Game game) {
    return Objects.nonNull(date) && Objects.nonNull(game);
  }

  public LocalDate getDate() {
    return this.date;
  }

  public Game getGame() {
    return this.game;
  }
}
```

With this implementation, it is now impossible to have an instance of `Playthrough` at
runtime that contains 'bad' data. We can now make use of functions like
`determineMostRecent` (described earlier) without adding any guards or fearing any bugs.
How nice!

Even so, is it really appropriate to make use of checked exceptions in this way? After
all, conventional wisdom says that [exceptions shouldn't be used for control flow, in
part because throwing exceptions is quite expensive in most languages, including
Java](https://wiki.c2.com/?DontUseExceptionsForFlowControl). It sure seems like this
approach walks, if not crosses, this line. And, as much as I wish it weren't the case,
in my experience, bad data is usually not that exceptional; hence the use of exceptions
to filter out bad data seems like an abuse of this language feature.

### Optionals to the rescue!

Ok, so using exceptions to guard against bad data probably isn't great, but what
recourse do we have? Barring the use of exceptions, the result of calling a constructor
is always initialization, which is exactly what we are trying to avoid when bad data is
provided. Fortunately, there is indeed another way, which leverages the established
[static factory method
pattern](https://www.baeldung.com/java-constructors-vs-static-factory-methods).

In short, the approach is this: ***by hiding our constructor (i.e. reducing its access
to `private`) behind a static factory method that returns our class wrapped inside an
`Optional`, we can indeed retain the ability to enforce data quality in our class***.
Here is how I would implement this pattern in the `Playthrough` class:

```java
public final class Playthrough {
  private final LocalDate date;
  private final Game game;

  /**
   * Returns a new Playthrough with the provided data
   *
   * <p>Returns an empty Optional if and only if the initialization data is
   * invalid
   */
  public static Optional<Playthrough> of(LocalDate date, Game game) {
    Playthrough raw = isValid(date, game) ? new Playthrough(date, game) : null;
    return Optional.ofNullable(raw);
  }

  private Playthrough(LocalDate date, Game game) {
    this.date = date;
    this.game = game;
  }

  /** The given data is valid if and only if both date and game are not null */
  private static boolean isValid(LocalDate date, Game game) {
    return Objects.nonNull(date) && Objects.nonNull(game);
  }

  public LocalDate getDate() {
    return this.date;
  }

  public Game getGame() {
    return this.game;
  }
}
```

In my own experience, this pattern has many benefits and few downsides. Most
importantly, it strongly enforces a contract that promises that any instance of
`Playthrough` will contain valid data. And, by leveraging Java's standard `Optional`
class, it imposes little overhead on anyone attempting to initialize new `Playthrough`s.

To illustrate the latter benefit, consider a situation where in my board game tracking
application I find myself with the need to zip together a list of dates and games into a
collection of `Playthrough`s:

```java
/**
 * Combines the provided dates and games pairwise into a list of playthroughs
 *
 * <p>All indices in the longer list without a corresponding index in the
 * shorter list are ignored
 */
public static List<Playthrough> zipIntoPlaythroughs(
    List<LocalDate> dates, List<Game> games) {
  int smallerListSize = Math.min(dates.size(), games.size());
  return IntStream.range(0, smallerListSize)
      .mapToObj(i -> Playthrough.of(dates.get(i), games.get(i)))
      .filter(Optional::isPresent)
      .map(Optional::get)
      .collect(Collectors.toList());
}
```

Using this pattern, it takes only two lines of code[^2] to successfully filter out the
bad data and unwrap the remaining `Optional`s, after which point I, the user of the
`Playthrough` class, never again have to worry about bad data in my instances. All this
without referencing a single exception!

In many cases, such as dealing with a data class like `Playthrough`, I highly recommend
this approach to initialization. It is a relatively lightweight pattern that, when set
up correctly, can help to eliminate the possibility of invalid, meaningless, or
otherwise bad data. Just like a cardboard box in a warehouse, a data class is not a
particularly flashy tool. But, by making sure it stays robust and durable, we also
ensure quality for the far more important stuff that it holds inside.

### Postscript: the bigger picture

Although I haven't personally found any other discussions on the specific pattern of
combining static factory methods with `Optional`s, many others before me have already
described similar concepts in other contexts. Most importantly, this pattern falls under
a much more wide-reaching principle of software development: "**make invalid states
impossible/unrepresentable**". That is, write code (and, in particular, construct types)
that makes it impossible (i.e. a compiler error) for invalid data to be representable.
This principle is often recommended within the world of functional programming (as seen
[here](https://youtu.be/IcgmSRJHu_8) and
[here](https://fsharpforfunandprofit.com/posts/designing-with-types-making-illegal-states-unrepresentable)).
I particularly recommend ["Type Safety Back and Forth" by Matt
Parsons](https://www.parsonsmatt.org/2017/10/11/type_safety_back_and_forth.html), and a
couple of blog posts
(["Parse, Don't Validate"](https://lexi-lambda.github.io/blog/2019/11/05/parse-don-t-validate)
and
["Types as Axioms"](https://lexi-lambda.github.io/blog/2020/08/13/types-as-axioms-or-playing-god-with-static-types))
by [Alexis King](https://lexi-lambda.github.io/about.html), which wonderfully explain
how the pattern described here merely scratches the surface when it comes to enforcing
constraints with types. These are powerful ideas that deserve further recognition
throughout the industry.

### Acknowledgements

Many thanks to [Eva Grench](https://www.linkedin.com/in/eva-grench) for helping workshop
the implementation of this pattern, and for proof-reading this blog post!

[^1]: The `Game` type, in turn, might start as an enumeration of all the games I have
    available to play:

    ```java
    public enum Game {
      SETTLERS_OF_CATAN,
      SPLENDOR,
      TERRAFORMING_MARS,
      WINGSPAN;
    }
    ```

[^2]: In comparison, here is the corresponding implementation of `zipIntoPlaythroughs`
    if the checked-exceptions-based approach is used:

    ```java
    public static List<Playthrough> zipIntoPlaythroughs(
        List<LocalDate> dates, List<Game> games) {
      int smallerListSize = Math.min(dates.size(), games.size());
      return IntStream.range(0, smallerListSize)
          .mapToObj(
              i -> {
                Playthrough raw = null;
                try {
                  raw = new Playthrough(dates.get(i), games.get(i));
                } catch (Playthrough.InvalidDataException ignored) {
                }
                return raw;
              })
          .filter(Objects::nonNull)
          .collect(Collectors.toList());
    }
    ```

    Not only does this require more lines of code than the other implementation, but
    also the intent of these additional lines is arguably much less clear.
