# computergeneration
[![computergeneration on crates.io](https://img.shields.io/crates/v/computergeneration.svg)](https://crates.io/crates/computergeneration)

computergeneration is a partial `compgen` replacement whose primary goal is to provide case-insensitive completions, since [`compgen` doesn't seem to be able to do that](https://unix.stackexchange.com/questions/204848/getting-case-insensitive-completions-with-compgen-in-bash). It's written in Rust, and you can install it right now:

```bash
cargo install computergeneration
```

```
$ computergeneration --help
computergeneration 0.1.0
Generates completions based on a word list and a prompt.

Word list is expected to be provided via stdin, and newline-delimited.

USAGE:
    computergeneration.exe [FLAGS] <input>

FLAGS:
    -i, --case-insensitive
            Whether matches should ignore case

    -h, --help
            Prints help information

    -V, --version
            Prints version information


ARGS:
    <input>
            Beginning of line to complete against
```

## Basic Usage
I had a sort of janky Bash completion script that looked like this with `compgen`:

```bash
# Jump to the machine's projects directory ($PROJ) and optionally a project
# inside it.
function proj() {
	cd "$PROJ/$1"
}

function _proj_complete() {
	COMPREPLY=( $(compgen -W "$(\ls -1 $PROJ)" "${COMP_WORDS[1]}") )
	return 0
}
complete -F _proj_complete proj
```

I replaced it with this, using computergeneration:

```bash
# Jump to the machine's projects directory ($PROJ) and optionally a project
# inside it.
function proj() {
	cd "$PROJ/$1"
}

function _proj_complete() {
	COMPREPLY=( $(\ls -1 $PROJ | computergeneration -i "${COMP_WORDS[1]}") )
	return 0
}
complete -F _proj_complete proj
```

Suddenly, I'm able to tab-complete project names and move into them even when I forget how they're capitalized!

This is made a lot less useful by the fact that I also use [fzf-tab-completion](https://github.com/lincheney/fzf-tab-completion), but it was a nice exercise.

## Where did this name come from?
Imagine you spelled out "comp gen" and used tab-completion to finish each word... but you got the wrong word both times!

## License
Licensed under the MIT license. See [LICENSE.txt](LICENSE.txt) or <http://opensource.org/licenses/MIT> for details.