#!/usr/bin/env node

"use strict";

import child_process from "child_process";
import { program } from "commander";
import path from "path";

import Logger from "./logger.mjs";

const PREDICTABLE_PATH_TO_RS_EXECUTABLE = "node_modules/rsrv/crates/rsrv";
const BANISHED_CHARS = [";", "&", "|", "[", "]", "{", "}", "(", ")", ".."];

program
  .name("rsrv")
  .description("A simple, fast command line file server, implemented in rust.")
  .version("1.1.0");

program.argument(
  "<directories...>",
  "A space delimited list of directories you wish to serve files from"
);

program.option(
  "-p, --port <number>",
  "Specify a custom port to bind the server instance to. Default is 8080",
  8080
);
program.option(
  "--cors <string>",
  "Enable CORS, sets `Access-Control-Allow-Origin` to `*`, or a provided pattern",
  false
);
program.option("--fallback", "Rewrite all not-found requests to `index.html`", false);
program.option(
  "--log-level <string>",
  "Can be used to control output of server logs. Levels are: Info, Warn, Error, Fatal, Silent. Default is: Info",
  "Info"
);
program.option("--no-compression", "Do not compress outgoing files", false);
program.option(
  "--no-port-switching",
  "Do not open a port other than the one specified when it's taken",
  true
);

program.parse();

const programOptions = program.opts();
const programArguments = program.args;

const logLevel = programOptions.logLevel;

const stdout = new Logger({ level: logLevel.toLowerCase() });

const pathToExecutable = path.resolve(process.cwd(), PREDICTABLE_PATH_TO_RS_EXECUTABLE);

let formattedExecutableCommand = `${pathToExecutable}`;
formattedExecutableCommand += ' ';

for (let directoryArg of programArguments) {
	for (const banishedChar of BANISHED_CHARS) {
		directoryArg = directoryArg.replace(banishedChar, "");
	};
	formattedExecutableCommand += `--dir=${directoryArg} `;
}

formattedExecutableCommand += ` --port=${programOptions.port}`;

stdout.info({ programOptions, programArguments, formattedExecutableCommand });

const child_fork = child_process.exec(formattedExecutableCommand, (e, stdo, stde) => {
	if (e) {
		stdout.error(e);
	}

	stdout.info(stdo);
	stdout.error(stde);
});

child_fork.stdout.on('data', (data) => {
	process.stdout.write(data);
})

child_fork.stderr.on('data', (data) => {
	process.stderr.write(data);
})
