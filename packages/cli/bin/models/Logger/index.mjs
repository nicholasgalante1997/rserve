import fs from "node:fs";
import chalk from "chalk";
import { optional } from "../../utils/option.mjs";
import { safeStringify } from "../../utils/stringify.mjs";

const LEVEL_MAP = new Map([
  ["info", 0],
  ["debug", 0],
  ["warn", 1],
  ["error", 2],
  ["fatal", 2],
  ["silent", 3]
]);

function Logger(options = {}) {
  const { level = "info", transport = { type: "stdout" }, name } = options;
  this.level = level;
  this.transport = transport;
  this.name = name;

  const commitLogTransaction = (log, level) => {
    if (LEVEL_MAP.has(level)) {
      if (LEVEL_MAP.get(level) >= LEVEL_MAP.get(this.level)) {
        if (this.transport.type === "file") {
          optional(fs.appendFileSync.bind({}, this.transport.destination, log));
        } else {
          switch (level) {
            case "info": {
              console.info(
                `${chalk.hex("80acf8").bold("[INFO]")}: ${chalk.hex("80acf8")(safeStringify(log))}`
              );
              break;
            }
            case "debug": {
              console.debug(
                `${chalk.hex("78dbcb").bold("[DEBUG]")}: ${chalk.hex("78dbcb")(safeStringify(log))}`
              );
              break;
            }
            case "warn": {
              console.warn(
                `${chalk.hex("ffe76e").bold("[WARN]")}: ${chalk.hex("ffe76e")(safeStringify(log))}`
              );
              break;
            }
            case "error": {
              /** Fall through to "fatal" handling. */
            }
            case "fatal": {
              console.error(
                `${chalk.hex("ff4e52").bold("[ERROR]")}: ${chalk.hex("ff4e52")(safeStringify(log))}`
              );
              break;
            }
            default: {
              console.log(
                `${chalk.hex("80acf8").bold("[LOG]")}: ${chalk.hex("80acf8")(safeStringify(log))}`
              );
            }
          }
        }
      }
    }
  };

  this.info = (message) => commitLogTransaction(message, "info");
  this.debug = (message) => commitLogTransaction(message, "debug");
  this.warn = (message) => commitLogTransaction(message, "warn");
  this.error = (message) => commitLogTransaction(message, "error");
  this.fatal = (message) => commitLogTransaction(message, "fatal");
}

export default Logger;
