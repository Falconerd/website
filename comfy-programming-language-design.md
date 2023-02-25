---
title: Designing A Comfy Programming Language
subtitle: 2023-25-02
---

What is it that programmers really want out of a language? For myself, it's **ease of use** and **compilation speed**.

In this article I will cover Rust, Zig, and C to show how they fail to meet my requirements.

## Ease Of Use

Ease of use covers a broad range of topics such as:

- Installing a language.
- Compiling a program with it.
- The tooling surrounding it.
- How useful the Standard Library is.
- And more...

Of course the language itself should be straightforward and have features that are obviously the correct ones to use for a particular problem.

The second most important skill for a programmer after problem solving is writing readable code. If nobody after you - or your future self - cannot easily read your code it will bring development to a stand-still.

## Rust

Rust is the newest language I have picked up, and it's the closest to my ideal for ease-of-use, I am suspicious of the npm-like package management that is going on.

While writing this article, I decided to upgrade the website. It was running on a Rust framework called Dioxus v0.2. The migration process was horrible because the devs pushed out v0.3 with breaking changes and no update to the documentation (this has nothing to do with Rust itself, it was just shit).

and is now just Markdown files + a shell script.




```
sum = fn(x: number, y: number) -> number {
        return x + y
}
```

I'm no stranger to switching languages, in [my last post](#) I told a story about how I switched between five languages on my current game project.


