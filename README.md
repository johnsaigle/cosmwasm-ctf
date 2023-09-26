# Oak Security Capture The Flag ⛳️ - AwesomWasm Online Event

Follow us on Twitter at [@SecurityOak](https://twitter.com/SecurityOak) to receive the latest information on the event. All the details will be first published there and then compiled in this `README`.

The CTF ended on July 17th. Thanks to everyone for participating in it!

## Writeups

1. [Capture The Flag ️Writeups — AwesomWasm 2023 Pt. 1](https://medium.com/oak-security/capture-the-flag-%EF%B8%8Fwriteups-awesomwasm-2023-pt-1-a40c6e506b49)
2. [Capture The Flag ️Writeups — AwesomWasm 2023 Pt. 2](https://medium.com/oak-security/capture-the-flag-%EF%B8%8Fwriteups-awesomwasm-2023-pt-2-cb3e9b297c0)

## Navigation

1. To get started with the challenges, please visit the [main](https://github.com/oak-security/cosmwasm-ctf/tree/main) branch.
2. To view the proof of concept for the challenges, please visit the [poc-exploit](https://github.com/oak-security/cosmwasm-ctf/tree/poc-exploit) branch. The proof of concept is written as an `exploit()` test case and can be found in the `exploit.rs` file.
3. To view the fixed versions of the challenges, please visit the [fixed](https://github.com/oak-security/cosmwasm-ctf/tree/fixed) branch. All proof of concept test cases are prefixed with `#[ignore="bug is patched"]`, so they will not be automatically executed when running `cargo test`.

## Winners

1. [@forbiddnstars](https://twitter.com/forbiddnstars)
2. [@CruncherDefi](https://twitter.com/CruncherDefi)
3. [@jc0f0116](https://twitter.com/jc0f0116)
4. [@LeTurt_](https://twitter.com/LeTurt_)
5. [@i_be_jc](https://twitter.com/i_be_jc)

Note that the three best submissions had all identified all security vulnerabilities. We used the quality and readability of the report as a tie-breaker to determine the winner. 🎉

Here is the [official announcement](https://twitter.com/SecurityOak/status/1684462534244327424) in Twitter.

## Key information

Our Capture The Flag event will be exclusively centered around CosmWasm smart contract security. As a participant, you will review the code of several smart contracts. To crack the challenges you will need to spot a wide variety of dangerous security vulnerabilities. 

The top 5 winners will be announced on [Twitter](https://twitter.com/securityoak) and receive shoutouts there.

- **Number of challenges:** 10
- **Start date:** 10th of July, 2023
- **End Date:** 17th of July, 2023 (23:59)
- **Location:** Worldwide! our event will be held online and asynchronously so anyone can participate.
- **Recorded live stream** : https://www.youtube.com/live/YIb3UsLxlbQ?feature=share
- **Results submission**: [Google Forms link](https://docs.google.com/forms/d/e/1FAIpQLSfc5Pr7sNCOIUP4aORM9JV4MTJi0Kl7QhPLHQSHX8Bgb9BUCw/viewform)

Crack all our challenges and show the community that you know your way in security, either as an auditor or a security-minded developer!

Join the [official Telegram channel](https://t.me/+8ilY7qeG4stlYzJi) to get in touch with us!

Follow us on [Twitter](https://twitter.com/SecurityOak) to stay up to date.

## Getting started

### Quick links

- [POINTS.md](POINTS.md) contains the overall points of each challenges.
- [SAMPLE_REPORT_TEMPLATE.md](SAMPLE_REPORT_TEMPLATE.md) contains a sample report template in Markdown format.

### Running test case

1. Navigate into challenge folder.

```bash
cd ctf-01/
```

2. Run tests

```bash
cargo test
```

## Questions?

Just open an issue in this repository to get an answer from our team.
