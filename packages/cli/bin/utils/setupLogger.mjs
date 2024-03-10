/**
 *
 * @param {string} logLevel
 * @returns {Logger}
 */
export function setupLogger(logLevel = "Silent") {
  return new Logger({ level: logLevel.toLowerCase() });
}
