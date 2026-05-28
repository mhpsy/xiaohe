# xiaohe

A small CLI for querying the **Xiaohe Shuangpin (小鹤双拼)** input scheme. It maps pinyin syllables to their two-key Shuangpin codes and back, prints the scheme tables, and offers did-you-mean suggestions for typos.

This is a **learning / lookup tool**, not an input method engine. It does not handle Chinese characters or multi-syllable text.

## Install

```bash
cargo install --git https://github.com/mhpsy/xiaohe
```

## Usage

```bash
$ xiaohe encode xiao         # alias: e
xn

$ xiaohe decode xn           # alias: d
xiao

$ xiaohe table               # ASCII keyboard layout, colored on a TTY

$ xiaohe list --filter x     # all syllables starting with x
xi	xi
xia	xx
xian	xm
xiang	xl
xiao	xn
...

$ xiaohe interactive         # alias: i — encode one syllable per line
xiaohe interactive — type a pinyin syllable, Ctrl-D or 'quit' to exit
hao
hc
xian
xm
```

Use `v` to type `ü`: `xiaohe encode nv` → `nv` (= nü).

Zero-initial syllables follow the Xiaohe rule that the initial key is the first letter of the final: `xiaohe encode ang` → `ah`.

Invalid input gets a friendly hint:

```bash
$ xiaohe encode xio
error: 'xio' is not a legal pinyin syllable
hint: did you mean 'xi', 'xia', 'xiao'?
```

## Why

The Xiaohe scheme has ~35 final mappings to memorize. This CLI is a fast offline lookup so you can drill the table from the terminal.

## License

MIT. See [LICENSE](./LICENSE).

Xiaohe Shuangpin scheme designed by 何海峰. This tool is independent and unaffiliated.
