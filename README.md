# ORCID Fetcher CLI
A third-party CLI toolkit for downloading public data from the ORCID API.

## Features
- Fetch **all Work Details** for a given ORCID iD (v 3.0 API)
- Pretty-printed JSON written to any directory you choose
- Safe "fetch-only-when-changed" logic
  - compares existing work-details with the latest summaries and downloads *only* new or updated entries; no file rewrite when unchanged
  - ideal for static hosting (e.g. GitHub Pages)

(*initial release: Work Details only – more record types will follow*).

## Installation
### Dependencies
- Rust ≥ 1.88
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

### Build
```bash
git clone https://github.com/hyperfinitism/orcid-fetcher
cd orcid-fetcher
cargo build --release
```

## Usage

### `orcid-works-cli`
#### Command
```bash
orcid-works-cli --id $ORCID_iD [Options]
```

#### Options
| Flag | Description | Default |
| :--- | :---------- | :------ |
| `-i`, `--id` \<String\> | ORCID iD (e.g. `3141-5926-5358-9793`) | *(required)* |
| `-o`, `--out` \<PathBuf\> | Output JSON file path (parent dirs auto-created) | `./output.json` |
| `--concurrency` \<usize\> | Maximum parallel requests (1-32). Should not exceed rate-limit. | `8` |
| `--rate-limit` \<u32\> | Requests-per-second cap (1–40). See [Guidelines](#guidelines) section. | `12` |
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
    --id 3141-5926-5358-9793 \
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