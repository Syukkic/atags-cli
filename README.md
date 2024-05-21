atags-cli is command line tool encapsulating partial functions of [audiotags](https://docs.rs/audiotags/latest/audiotags) for editing audio metadata.

```bash
Audio metadata command line tool encapsulating partial functions of audiotags

Usage: atags-cli [OPTIONS] <NAME>

Arguments:
  <NAME>  File Path

Options:
  -t, --title <TITLE>                Set audio title
  -a, --artist <ARTIST>              Set audio artist
  -g, --genre <GENRE>                Set audio genre
  -c, --composer <COMPOSER>          Set audio composer
      --track-number <TRACK_NUMBER>  Set audio track number
      --album-title <ALBUM_TITLE>    Set audio album title
  -h, --help                         Print help
  -V, --version                      Print version
```

### Examples

```bash
atags-cli ./assets/rm1210.m4a -t "rm1210" -a "My Little Airport"
atags-cli ./assets/rm1210.m4a
```
