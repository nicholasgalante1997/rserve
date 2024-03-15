#!/usr/bin/env node

"use strict";

import child_process from "child_process";
import { program } from "commander";
import path from "path";

import { ExecutableBuilder, HostConfig, Logger } from "./models/index.mjs";
import { sanitizeArgs, setupLogger } from './utils/index.mjs';

const PREDICTABLE_PATH_TO_RS_EXECUTABLE = "node_modules/rsrv/crates/rsrv";
const DEFAULT_PORT = 8080;

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
  DEFAULT_PORT
);

program.option(
  "--cors <string>",
  "Enable CORS, sets `Access-Control-Allow-Origin` to `*`, or a provided pattern",
  false
);

program.option(
  "--cache-control <number>",
  "Set a custom cache control header time. Default is 3 days.",
  259200
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

const logger = setupLogger(programOptions.logLevel);
const executable = getExecutable(programArguments, programOptions);
const hostConfig = new HostConfig();

run(executable);

/**
 *
 * @param {Array<string>} commandLineArguments
 * @param {Object} options
 * @returns {string}
 */
function getExecutable(commandLineArguments, options = { port: DEFAULT_PORT }) {
  if (commandLineArguments.length === 0) {
    logger.error("Missing arguments array.");
    process.exit(2);
  }

  const pathToRsrvExecutable = path.resolve(process.cwd(), PREDICTABLE_PATH_TO_RS_EXECUTABLE);
  const sanitizedArgs = sanitizeArgs(commandLineArguments);
  const executableBuilder = new ExecutableBuilder();

  executableBuilder.setDirectories(sanitizedArgs);
  executableBuilder.setPath(pathToRsrvExecutable);
  executableBuilder.setPort(options.port);
  executableBuilder.setLogLevel(options.logLevel);

  if (options.cors) {
    executableBuilder.setCors(options.cors);
  }

  if (options.noCompression) {
    executableBuilder.setNoCompression(options.noCompression);
  }

  if (options.noPortSwitching) {
    executableBuilder.setNoPortSwitching(options.noPortSwitching);
  }

  if (options.fallback) {
    executableBuilder.setFallback(options.fallback);
  }

  return executableBuilder.build().format();
}

/**
 *
 * @param {string} executable
 * @returns {void}
 */
function run(executable) {
  const child_fork = child_process.exec(executable, (e, stdout, stderr) => {
    if (e) {
      logger.error(e);
    }

    logger.info(stdout);
    logger.error(stderr);
  });

  child_fork.stdout.on("data", (data) => {
    process.stdout.write(data);
  });

  child_fork.stderr.on("data", (data) => {
    process.stderr.write(data);
  });
}
