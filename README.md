<h1 align="center">rsget</h1>

<h3 align="center">A fast async CLI file downloader written in Rust</h3>

<div align="center">
[Usage](#usage) •
[Installation](#installation) •
[Configuration](#configuration)
</div>

---

## Usage

```sh
rsget https://example.com/file.zip               # Save with filename derived from URL

rsget https://example.com/file.zip -o myfile.zip # Specify output filename
```

---

## Installation

- Build from source
```sh
git clone https://github.com/whyhilde/rsget.git
cd rsget
just install
```

---

## License

© 2026 whyhilde - Licensed under the MIT License. See [LICENSE](LICENSE) for details.
