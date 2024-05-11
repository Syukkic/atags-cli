atags-cli is command line tool encapsulating partial functions of [audiotags](https://docs.rs/audiotags/latest/audiotags) for editing audio metadata.

```bash
atags-cli -h
Audio metadata command line tool encapsulating partial functions of audiotags

Usage: atags-cli [COMMAND]

Commands:
  show  Show audio exiting metadata
  set   Set audio metadata
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

```bash
atags-cli set --help
Set audio metadata

Usage: atags-cli set [OPTIONS] <FILE>

Arguments:
  <FILE>

Options:
  -t, --title <TITLE>                Set audio name
  -a, --artist <ARTIST>              Set audio artist(s)
  -g, --genre <GENRE>                Set audio genre
  -c, --composer <COMPOSER>          Set audio composer
      --track-number <TRACK_NUMBER>  Set audio track number
      --album-title <ALBUM_TITLE>    Set audio album title
  -h, --help
```

```bash
Show audio exiting metadata

Usage: atags-cli show <FILE>

Arguments:
  <FILE>

Options:
  -h, --help  Print help
```

### Examples

```bash
atags-cli set ./assets/rm1210.m4a -t "rm1210" -a "My Little Airport"
atags-cli show ./assets/rm1210.m4a
```
