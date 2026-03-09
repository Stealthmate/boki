# boki

Boki is a language/toolchain for doing [plain text accounting](https://plaintextaccounting.org/) (PTA).

Boki was deeply inspired by a similar PTA tool called [hledger](https://hledger.org/), however boki is less focused on immediate usefulness and more focused on providing a solid foundation that other tools can build on. What this means in practice, can be summarized like so:

#### Make it easy for humans to read and write the data.

Plain text accounting is a way for _humans_ to do accounting without enterprise-grade software. Boki aims to make it easy for humans to both manipulate data, as well as record it. In practice this means having a format which is _concise_ - in very simple terms, this means requiring the least amount of characters to type without compromising readability. For example, YAML can be considered more concise compared to JSON.

#### Make it easy for other software to process the data.

Accounting can get complicated. Calculating your bank balance at the end of the year is easy enough. Figuring out your actual living expenses and whether they're appropriate or not requires some more custom-built calculations which a generic tool may not be able to provide.

Boki aims to provide _interoperability_ - it ensures that everything you have recorded is correct, and it converts your data to a well-structured format (JSON). How you aggregate that is entirely up to you - you could use `jq` or `Python` or whatever other tool to perform whatever analysis you like on your data.

#### Make it easy to record metadata.

At its very core, accounting data is about the flow of money. This is expressed via the notion of an _account_, along with the technique of [double-entry bookkeeping](https://en.wikipedia.org/wiki/Double-entry_bookkeeping). Naturally then, PTA tools' main objective is to provide a way to do exactly that - record double-entry transactions and make sure they're correct.

However, in practice, accounting data by itself is not really that useful. Accounts are just buckets, and transactions are just a way of saying "move X amount of money from bucket A to bucket B". On the other hand, sometimes you want to know more than that - whether a transaction was completed online, whether it was part of your honeymoon or just the monthly electricity bill, etc. The more complicated analysis you want to perform, the more data you need for it.

Because of this, boki treats metadata as a first-class citizen - it gives you the ability to record arbitrary, structured metadata as part of all your transactions, all while giving you the flexibility of a concise syntax.

## HOW-TOs

### How to release a new version

1. Decide the next version number (e.g. `v1.2.3`).
2. Add a new section to the top of [`docs/release-notes.md`](./docs/release-notes.md), describing the changes from the previous version. Commit this.
3. Update the binary version in `Cargo.toml`. Commit this.
4. Create a tag with the specified version, e.g. `git tag -a v1.2.3 -m 'see docs/release-notes.md'` and push the tag.
