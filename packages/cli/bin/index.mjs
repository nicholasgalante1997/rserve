#!/usr/bin/node

"use strict";

import { program } from "commander";
import Logger from "./logger.mjs";

program
  .name("rserve")
  .description("A simple, fast command line file server, implemented in rust.")
  .version("0.0.1");

program.argument(
  "<directories...>",
  "A space delimited list of directories you wish to serve files from",
);

program.option(
  "-p, --port <number>",
  "Specify a custom port to bind the server instance to. Default is 8080",
  8080,
);
program.option(
  "--cors <string>",
  "Enable CORS, sets `Access-Control-Allow-Origin` to `*`, or a provided pattern",
  false,
);
program.option(
  "--fallback",
  "Rewrite all not-found requests to `index.html`",
  false,
);
program.option(
  "--log-level <string>",
  "Can be used to control output of server logs. Levels are: Info, Warn, Error, Fatal, Silent. Default is: Info",
  "Info",
);
program.option("--no-compression", "Do not compress outgoing files", false);
program.option(
  "--no-port-switching",
  "Do not open a port other than the one specified when it's taken",
  true,
);
program.option(
  "--platform <string>",
  "Which rust binary executable to use based on which platform your code is running on. Default is x86_64-unknown-linux-gnu",
  "x86_64-unknown-linux-gnu",
);

program.parse();

const programOptions = program.opts();
const programArguments = program.args;

const logLevel = programOptions.logLevel;

const stdout = new Logger({ level: logLevel.toLowerCase() });

stdout.info({ programOptions, programArguments });
