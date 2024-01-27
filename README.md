# clap-test-rs

Repo for testing Rust clap crate.

## Usage

Clone repo, then `cargo build`, which will create a binary at `target/debug/ctr`.

Ensure that binary is in your PATH. An easy solution is to copy it to `/usr/local/bin`.

Ensure that `ctr-asdfblah` is in your PATH. An easy solution is to copy it to `/usr/local/bin`.

When both `which ctr` and `which ctr-asdfblah` return a path, you're ready to go.

Run the test via `ctr asdfblah arg1 arg2`. Your output should look like:

```
from ctr binary :: Calling out to "ctr-asdfblah" with ["arg1", "arg2"]
from ctr-asdfblah :: hello from executable file 'ctr-asdfblah' :)
from ctr-asdfblah :: this file received the args: arg1 arg2

from ctr-asdfblah :: about to call 'ctr clone AsdfBlah_ITS_ALIVE'
from ctr-asdfblah :: ctr call starting...

from ctr binary :: Cloning AsdfBlah_ITS_ALIVE

from ctr-asdfblah :: ...ctr call ended
from ctr-asdfblah :: goodbye from executable file 'ctr-asdfblah'  o/
```

This output shows the ctr binary calling out to the ctr-asdfblah binary,
which then calls back to the ctr binary.
This pattern allows users to write their own extensions to the ctr app.
This is how the external-process of `git` works; i.e., I can create a file on my path called
`git-foo` and then call `git foo` and the git binary will call out to my `git-foo` binary.
That `git-foo` binary can then do whatever I need, including a call back to the original `git`.
This allows something like `git-co` to be written, which is a common alias for `git checkout`.
The `git-co` file would simply execute `git checkout $@` to pass along any arguments it received.
Instant alias, but with the option of more fully powered extensions, and not _just_ an alias.

Meanwhile, if an invalid command is passed to the ctr binary, it will print an error and exit.
For example, if I do `which ctr-fail` on my current system, I get:

```
ctr-fail not found
```

So, there is no `ctr-fail` binary on my PATH. If I try `ctr fail`, then I get:

```
from ctr binary :: ctr: 'fail' is not a ctr command. See 'ctr --help'.
```

This message mirrors git's error message when an invalid command is passed to it.
Obviously, one would remove the `from ctr binary ::` part of the message, but that's just for demo.
