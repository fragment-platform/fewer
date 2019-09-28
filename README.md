# Fewer

It's like HTML, but Fewer.

## Usage

`./cargo run -- input.html output.html

Here's the tag whitelist:

```
title

h1, h2, h3, h4, h5, h6

p, br

a (href, title), img (src, alt)

strong, em, blockquote, pre

ol, ul, li

hr

table, tr, th, td
```