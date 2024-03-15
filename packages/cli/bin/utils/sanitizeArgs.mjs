/**
 * 
 * @param {string[]} rawArgs 
 * @returns {string[]}
 */
export function sanitizeArgs(rawArgs) {
  const BANISHED_CHARS = [";", "&", "|", "[", "]", "{", "}", "(", ")", ".."];
  let cleanArgs = [];
  for (const banishedChar of BANISHED_CHARS) {
    for (const rawArg of rawArgs) {
      cleanArgs.push(rawArg.replace(banishedChar, ""));
    }
  }
  return cleanArgs;
}
