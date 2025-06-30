# ORCID Fetcher CLI

A third-party CLI toolkit for downloading public data from the ORCID API  
(*initial release: Work Details only – more record types will follow*).

## Features
- Fetch **all Work Details** for a given ORCID iD (v 3.0 API)
- Pretty-printed JSON written to any directory you choose
- Safe "write-only-when-changed" logic (ideal for Git repos)

## Build

```bash
git clone https://github.com/hyperfinitism/orcid-fetcher
cd orcid-fetcher
cargo build --release
```

## Usage

### `orcid-works-cli`
#### Usage
```bash
orcid-works-cli \
    --id 3141-5926-5358-9793 \
    --out ./output.json
```

#### Flags
| Flag | Description | Default |
| :--- | :---------- | :------ |
| `-i`, `--id` \<string\> | ORCID iD (e.g. `3141-5926-5358-9793`) | *(required)* |
| `-o`, `--out` \<path\> | Output JSON file path (parent dirs auto-created) | `./output.json` |
| `--concurrency` \<usize\> | Maximum parallel requests | `10` |
| `--rate-limit` \<u32\> | Requests-per-second cap (1–40) | `12` |
| `--user-agent-note` \<string\> | Text appended to the built-in User-Agent string | *(none)* |
| `-h`, `--help` | Print help | — |
| `-V`, `--version` | Print version | — |

#### Defaults legend

* **(required)** – the option must be provided  
* **(none)** – optional, nothing is sent if omitted  
* any other value – used as default when the flag is absent

## ⚠️ Guidelines
Please respect ORCID's Public API policies:

- **Rate limits**: Do not exceed **12 requests/second** (burst up to 40/s)
- **Usage quotas**: Do not exceed **25k reads/day**
(per IP address)
- **Polling**: Avoid continuously polling the API for changes

See ORCID's References for more details.

## ❗ Disclaimer
This is a third-party tool and is not affiliated with, sponsored by, or endorsed by ORCID. It uses the ORCID Public API only to fetch public data. See ORCID’s *Public APIs Terms of Service* for complete usage rules.

## Contributing
PRs and issues are welcome—please open an issue first to discuss major changes.

## References
- ORCID ― [ORCID-Model](https://github.com/ORCID/orcid-model)
- ORCID ― [Public APIs Terms of Service](https://info.orcid.org/public-client-terms-of-service)
- ORCID ― [What are the API usage quotas and limits?](https://info.orcid.org/ufaqs/what-are-the-api-limits)

## License
Licensed under the Apache-2.0 License. See [LICENSE](LICENSE) for details.