# Luminator

A script that takes RGB hex codes like `a0df00` and prints a luminance value
for them.  Like the Y in YIQ.

## luminator-rs

I did this in Python centuries ago, for determining accessibility of contrast
on a long-dead theme for my website.  _This_ version is rewritten in Rust.

## Standards

This uses the [luma](https://en.wikipedia.org/wiki/Luma_(video)) coefficients
from Rec. 601 (CCIR 601), that is:

    Y'[601] = 0.299*R' + 0.587*G' + 0.114*B'

I did not know about BT. 709 at the time, and there are no plans to support
it.

# License

Apache 2.
