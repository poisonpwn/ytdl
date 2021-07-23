# ytdl

a very very simple wrapper around youtube-dl and eyeD3


USAGE:
    ytdl [FLAGS] [OPTIONS] \<FILEPATH\> \<URL\> [-- \<youtube-dl_args\>...]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    show verbose output

OPTIONS:

    -A, --album <album\>        album to embed into id3v2 tag,
                               file format has to support id3v2

    -a, --artist \<artist\>      artist to embed into id3v2 tag, 
                               file format has to support id3v2

    -q, --quality \<quality\>    quality passed in to youtube-dl (defaults to best)

ARGS:

    <FILEPATH>              the output filepath
    <URL>                   URL of youtube video or search term to search for on youtube
    <youtube-dl_args>...    args passed to youtube-dl