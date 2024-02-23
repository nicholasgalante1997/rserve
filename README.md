# Rstatic Web Server

This is a Rust implementation of a static file server. This crate will produce an executable program that accepts command line configuration to indicate what file it should serve in the file system.

What does it need to know?

Needs (--flags still):

- Directory(ies)

Optional

- Port
- Http vs Https

Nice to have (--flags):

- Header Configuration
- file extension matching, auto-populating
- custom error pages


Command line format:

cargo run -- platform --dir=public --dir=assets --header="KEY=VALUE" --match="html,htm" --server-exception-error-page=500.html

So when it has that information, what does it need to do?

Check that it has been passed directories
Check that the directories it has been passed is valid
