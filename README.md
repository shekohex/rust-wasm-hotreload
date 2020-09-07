## Rust + WASM = 🔥 Hot-Reload 🔥

A PoC of using WebAssemply as a Hot-Reloadable Code logic at Runtime without restarting the Host Process.

for a simple demo see this [tweet](https://twitter.com/ShekoHex/status/1302973994417651714)

## Try it

First make sure you have Rust installed and nodejs (also NPM).

1. Clone

```bash
$ git clone https://github.com/shekohex/rust-wasm-hotreload
```

2. Build

```bash
$ npm install
$ npm run asbuild:optimized
$ cargo build
```

3. Run

in one terminal start build an watch for changes to our AssemplyScript File.

```bash
$ npm run watch
```

in another terminal simply run

```bash
$ cargo run
```

Now, just change the logic in the `assembly/index.ts` file and watch the magic happens :).

### Use Cases

This Project uses [Rust](https://www.rust-lang.org/) and [AssemblyScript](http://assemblyscript.org/) to address these use cases:

- Scripting (by extending your program with other functionality by using WASM files).
- GameDev (again, it would be useful to add some game logic like (NPCs, Quests, ..etc) as WASM module for easy changes and fast development)
- User-Land Extentions.
- Your Case?

### FAQ

1. Why using [AssemplyScript](https://www.assemblyscript.org/) and Not Rust for WASM?

Well, let's be honest here .. Rust Compile-Time will kill the whole idea about Fast Hot Reloading, and while AssemblyScript Compiles so fast .. it emits smaller code than Rust would and yet almost the same performance and that makes it a better option for writing code that compiles to WASM.

### Contributing

Want to join us? take a look at some of these issues:

- [Issues labeled "good first issue"][good-first-issue]
- [Issues labeled "help wanted"][help-wanted]

[good-first-issue]: https://github.com/shekohex/rust-wasm-hotreload/labels/good%20first%20issue
[help-wanted]: https://github.com/shekohex/rust-wasm-hotreload/labels/help%20wanted

### License

<sup>
Licensed under <a href="LICENSE">MIT license</a>.
</sup>

<br/>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the MIT license, shall
be licensed as above, without any additional terms or conditions.
</sub>
