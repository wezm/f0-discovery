<!-- [![Build status](https://travis-ci.org/japaric/f3.svg?branch=master)](https://travis-ci.org/japaric/f3) -->
<!-- [![crates.io](https://img.shields.io/crates/d/f3.svg)](https://crates.io/crates/f3) -->
<!-- [![crates.io](https://img.shields.io/crates/v/f3.svg)](https://crates.io/crates/f3) -->

# f0-discovery

Crate for interacting with STM32 F0 Discovery board. Most of the work was done by
[Jorge Aparicio] on his [f3] crate. Tested on [STM32F0308DISCOVERY].

[f3]: https://github.com/japaric/f3
[Jorge Aparicio]: https://github.com/japaric
[STM32F0308DISCOVERY]: http://www.st.com/en/evaluation-tools/stm32f0discovery.html

<blockquote class="twitter-tweet" data-lang="en"><p lang="en" dir="ltr">Proof it works 2/2 <a href="https://t.co/s9cCVoIXtp">pic.twitter.com/s9cCVoIXtp</a></p>&mdash; Wesley Moore (@wezm) <a href="https://twitter.com/wezm/status/790481366060183552">October 24, 2016</a></blockquote> <script async src="//platform.twitter.com/widgets.js" charset="utf-8"></script>

## Documentation

`cargo doc`

### Generating Peripherals

The code for the peripherals is mostly generated by the [svd2rust] tool. This
tool takes SVD files published by chip manufacturers and generates rust code to
interface with them.  For ST's chips the SVD files are available at
<https://cmsis.arm.com/vendor/stmicroelectronics/>. The [gen-peripherals.sh]
script can be used to regenerate the peripheral files.

[svd2rust]: https://github.com/japaric/svd2rust
[gen-peripherals.sh]: https://github.com/wezm/f0-discovery/blob/master/gen-peripherals.sh

## Examples

<https://github.com/wezm/f0-discovery/tree/master/examples>

**Note:** Not all have been ported to this board.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
