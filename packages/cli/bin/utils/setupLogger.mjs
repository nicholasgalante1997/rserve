import Logger from '../models/Logger/index.mjs';

/**
 *
 * @param {string} logLevel
 * @returns {Logger}
 */
export function setupLogger(logLevel = "Info") {
  return new Logger({ level: logLevel.toLowerCase() });
}
