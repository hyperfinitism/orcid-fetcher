# ORCID Fetcher CLI
A third-party CLI toolkit for downloading public data from the ORCID API.

## Features
- Fetch **all Work Details** for a given ORCID iD (v 3.0 API)
- Pretty-printed JSON written to any directory you choose
- Safe "fetch-only-when-changed" logic
  - compares existing work-details with the latest summaries and downloads *only* new or updated entries; no file rewrite when unchanged
  - ideal for static hosting (e.g. GitHub Pages)
- Multi-platform support

(*initial release: Work Details only – more record types will follow*).

## Installation

### Requirements
- [Rust (≥ 1.88)](https://www.rust-lang.org/) – needed only if you build from source.
- [jq (≥ 1.8.1)](https://jqlang.org/) – used by the example scripts in `examples/*`.

### Option 1: Pre-built binaries
Pre‑built binaries (≥ v0.2.1) are available on the [Releases page](https://github.com/hyperfinitism/orcid-fetcher/releases). The following platforms are currently supported:

| OS      | Arch   | File name pattern |
| :------ | :----- | :--- |
| Linux   | x86‑64 | `*-x86_64-unknown-linux-gnu.tar.gz` |
| macOS   | x86‑64 | `*-x86_64-apple-darwin.tar.gz` |
| macOS   | ARM64  | `*-aarch64-apple-darwin.tar.gz` |
| Windows | x86‑64 | `*-x86_64-pc-windows-msvc.zip` |

### Option 2: Build from source

```bash
# clone & build
git clone https://github.com/hyperfinitism/orcid-fetcher
cd orcid-fetcher
cargo build --release

# or install straight into ~/.cargo/bin
cargo install --git https://github.com/hyperfinitism/orcid-fetcher
```

## Usage

### `orcid-works-cli`
#### Command
```bash
orcid-works-cli --id $ORCID_ID [Options]
```

#### Options
| Flag | Description | Default |
| :--- | :---------- | :------ |
| `-i`, `--id` \<String\> | ORCID iD (e.g. `0000-0002-1825-0097`) | *(required)* |
| `-o`, `--out` \<PathBuf\> | Output JSON file path (parent dirs auto-created) | `./output.json` |
| `--concurrency` \<usize\> | Maximum parallel requests (1-32). Should not exceed rate-limit. | `8` |
| `--rate-limit` \<u32\> | Requests-per-second cap (1–40). See also [Guidelines](#guidelines) section. | `12` |
| `--user-agent-note` \<String\> | Text appended to the built-in User-Agent string | *(none)* |
| `--force-fetch` | Ignore diff and refetch every work-detail entry | `false` |
| `-h`, `--help` | Print help | — |
| `-V`, `--version` | Print version | — |

#### Defaults legend

* **(required)** – the option must be provided  
* **(none)** – optional, nothing is sent if omitted  
* any other value – used as default when the flag is absent

#### Example
```bash
orcid-works-cli \
    --id "0000-0002-1825-0097" \
    --out ./output.json
```

## Guidelines
Please respect ORCID's Public API policies:

- **Rate limits**: Do not exceed **12 requests/second** (burst up to 40/s)
- **Usage quotas**: Do not exceed **25k reads/day**
(per IP address)
- **Polling**: Avoid continuously polling the API for changes

See ORCID's [References](#references) for more details.

## Contributing
PRs and issues are welcome—please open an issue first to discuss major changes.

## Disclaimer
This is a third-party tool and is not affiliated with, sponsored by, or endorsed by ORCID. It uses the ORCID Public API only to fetch public data. See ORCID's [References](#references) for complete usage rules.

## References
- [ORCID/orcid-model - Github](https://github.com/ORCID/orcid-model)
- [Public APIs Terms of Service - ORCID](https://info.orcid.org/public-client-terms-of-service)
- [What are the API usage quotas and limits? - ORCID](https://info.orcid.org/ufaqs/what-are-the-api-limits)

## License
Licensed under the Apache-2.0 License. See [LICENSE](LICENSE) for details.