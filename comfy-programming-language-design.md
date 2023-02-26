---
title: Designing A Comfy Programming Language
subtitle: 2023-25-02
---

What is it that programmers really want out of a language? For myself, the most important aspect is **ease-of-use**.

An ideal language has the following attributes to make it easy to use:

<dl>
    <dt><strong>Simple installation</strong></dt><dd>Simply download a binary or install a package.</dd>
    <dt><strong>Fast compilation</strong></dt><dd>Quick turn-around time when programming is valuable.</dd>
    <dt><strong>Lack of footguns</strong></dt><dd>It should be difficult to have errors (besides logic errors).</dd>
    <dt><strong>Expected outcomes</strong></dt><dd>When you type a statement or expression, there should be no surprises about what it does.</dd>
    <dt><strong>Good tooling</strong></dt><dd>A Treesitter plugin or LSP is nice to have.</dd>
    <dt><strong>Useful Standard Library</strong></dt><dd>Having useful data structures and procedures in the standard library makes programming less tedious.</dd>
    <dt><strong>And more...</strong></dt>
</dl>

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


