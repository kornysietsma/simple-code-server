# Simple Code Server

This is a really simple local file server - part of my Polyglot Code tools - for the main documentation, see <https://polyglot.korny.info>

It's only 65 lines of code! So if you want to know more details, use the source (Luke).

## Intro

The only reason this exists is that most simple file servers have no CORS support - see a bit more on CORS at <https://enable-cors.org/>

When you run the Polyglot Code explorer in a browser, it now has the ability to display individual source code files as you click on them.  But to load the files, you need a tiny server to communicate to the JavaScript that is running in your web browser.

The code explorer itself can be run on a local server, like the ones [described at this Mozilla page](https://developer.mozilla.org/en-US/docs/Learn/Common_questions/set_up_a_local_testing_server).  And in theory you could load the code up in a very similar server, also on your local machine.

But CORS gets in the way - it is a specification designed to protect you from malicious scripts highjacking your browser and loading code from elsewhere on the net.  But in this case, it means if you have the Explorer running on `http://localhost:8080` and a file server on `http://localhost:8675` - the Explorer JavaScript can't actually make a call to the file server on port 8675.  (Some browsers historically allowed this, I'm not sure if they still do - but in general browsers should block this!)

## How to run it

I haven't yet uploaded binaries - when I have, you will be able to use the binary rather than `cargo`

For now, from the source directory, run:

~~~bash
cargo run -- path_to_files
~~~

This will run a web server on http://localhost:8675 with the root directory specified.

For example, to serve up the source code to the simple code server itself run:

~~~bash
cargo run -- .
~~~

Then you can open <http://localhost:8675/Cargo.toml> and see the contents of the `Cargo.toml` file.

*NOTE* this doesn't serve up directory indexes or anything useful - at the moment it's really aimed at programs like the Polyglot Code Explorer, not for manual use.

You can tweak the port, local interface, and CORS origins accepted - run `cargo run -- --help` for help on parameters, I'm not going to document them all here.

## Bypassing CORS - is that safe?

This *ONLY* bypasses calls from a server running on `localhost` or `127.0.0.1` or `0.0.0.0` - so it isn't useable at all by malicious scripts, only by scripts you host and run yourself.

You can add other valid origins by passing an `origins` parameter - you do so at your own risk; once you do this, any script running on a named origin could explore bits of your filesystem.

It should not be able to explore directories outside the one you specify, but I haven't tested this rigorously.

## License

Copyright © 2020 Kornelis Sietsma

Licensed under the Apache License, Version 2.0 - see LICENSE.txt for details
