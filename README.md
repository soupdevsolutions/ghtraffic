# ghTraffic

<img src="./img/logo_small.png" alt="ghTraffic logo" style="height: 200px; width:200px;"/>

[![CI](https://github.com/soupdevsolutions/ghstats/actions/workflows/ci.yml/badge.svg)](https://github.com/soupdevsolutions/ghstats/actions/workflows/ci.yml)
[![CD](https://github.com/soupdevsolutions/ghstats/actions/workflows/cd.yml/badge.svg?branch=main)](https://github.com/soupdevsolutions/ghstats/actions/workflows/cd.yml)

[ghTraffic](https://ghtraffic.com) is a small web application that allows you to view aggregated traffic information from multiple GitHub repositories.  

## Building and Testing

If you want to test the project locally, clone the repo and simply run:

```bash
cargo test
```

To build the binaries for AWS Lambda, you can use:

```bash
cargo lambda build --release --output-format zip
```
or use the Python script from the repo, which will also move the files to a destination that allows `opentofu` to pick them up:
```bash
chmod +x ./scripts/build.py
python3 ./scripts/build.py
```

## Contributing

Bug reports, as well as feature proposals, are more than welcome. Please create an issue first before attempting any fix or implementation.  

If you want to work on a specific issue, just ask :)  

You can also support the project on [buymeacoffee.com](https://www.buymeacoffee.com/mirch).
