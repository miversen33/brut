# Brut

This is an implementation of the [infinite monkey theorem](https://en.wikipedia.org/wiki/Infinite_monkey_theorem)

As opposed to that theorem, this will attempt to randomly generate [The Tragedy of Hamlet, Prince of Denmark](https://en.wikipedia.org/wiki/Hamlet)

## [y tho](https://i.kym-cdn.com/entries/icons/facebook/000/022/978/y_tho_meme.jpg)

Because it is funny

## How do I build this?

First you will need to install rust for your operating system: https://www.rust-lang.org/tools/install

Then, download the source
```bash
$ git clone https://github.com/miversen33/brut
```

Then build it
```bash
$ cd brut
$ cargo build --release
```

## How do I run this?

After you've successfully [built](#how-do-i-build-this) brut, you will need an [md5sum summed](https://en.wikipedia.org/wiki/Md5sum) hash that you want Brut to generate the source to. As an example (keeping with the Hamlet theme)

```bash
$ target/release/brut fb206c47affa38f6c58616fb912e280b # this hash is the md5sum of hamlet, found here: https://gist.githubusercontent.com/provpup/2fc41686eab7400b796b/raw/b575bd01a58494dfddc1d6429ef0167e709abf9b/hamlet.txt
```

NOTE: As the theorem states, this will likely never complete.

That said, Brut is a funny little guy. You can use him to crack other md5sums in the stupidest way possible (completely randomly guessing)

```bash
$ target/release/brut -c 2 $(echo -n "ab" | md5sum | sed -E 's/[- ]+$//g')
Attempting to find input that matches 187ef4436122d1cc2f40dc2b92f0eba0
Found match! Took 8829 monkies approximately 1 milliseconds
```
