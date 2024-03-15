/**
 * @param {Array<string>} directories An array of strings representing the directories to serve static files from
 * @param {string} path The path to the rust platform specific executable
 * @param {number} port The port to run the process on
 * @param {boolean|string} [cors=false] If a string is passed, CORS will be set to the passed string, if true is passed, CORS will be set to '*'
 * @param {boolean} [fallback=false] If set to true, will redirect file not found requests to index.html
 * @param {boolean} [noCompression=false] Do not send gzipped responses regardless of the incoming request "Accept-Encoding" header
 * @param {boolean} [noPortSwitching=false] If the port requested is in use, increment port until you find a free port. Setting this flag to false will disable this behavior
 * @param {string} [logLevel='info']  Server log level
 */
function Executable(
  path,
  directories,
  port = 8080,
  cors = false,
  fallback = false,
  logLevel = "info",
  noCompression = false,
  noPortSwitching = false
) {
  this.path = path;
  this.port = port;
  this.directories = directories;
  this.cors = cors;
  this.fallback = fallback;
  this.logLevel = logLevel;
  this.noCompression = noCompression;
  this.noPortSwitching = noPortSwitching;

  this.format = () => {
    let directoryArgs = directories.map((d) => `--dir=${d}`);
    let args = [this.path, ...directoryArgs, `--port=${this.port}`, `--log-level=${this.logLevel}`];
    if (this.cors) {
      args.push(`--cors=${typeof cors === "boolean" ? "*" : cors}`);
    }
    if (this.fallback) {
      args.push(`--fallback`);
    }
    if (this.noCompression) {
      args.push("--no-compression");
    }
    if (this.noPortSwitching) {
      args.push("--no-port-switching");
    }
    return args.join(" ");
  };
}

function ExecutableBuilder() {
  this.directories = null;
  this.path = null;
  this.port = 8080;
  this.cors = false;
  this.fallback = false;
  this.logLevel = "info";
  this.noCompression = false;
  this.noPortSwitching = false;

  this.setDirectories = (directories) => {
    this.directories = directories;
  };

  this.setPath = (path) => {
    this.path = path;
  };

  this.setPort = (port) => {
    this.port = port;
  };

  this.setCors = (cors) => {
    this.cors = cors;
  };

  this.setFallback = (fallback) => {
    this.fallback = fallback;
  };

  this.setLogLevel = (logLevel) => {
    this.logLevel = logLevel;
  };

  this.setNoCompression = (noCompression) => {
    this.noCompression = noCompression;
  };

  this.setNoPortSwitching = (noPortSwitching) => {
    this.noPortSwitching = noPortSwitching;
  };

  this.validate = () => {
    if (!this.directories) throw new Error("FatalConfigException: Missing directory arguments.");
    if (!Array.isArray(this.directories))
      throw new Error("FatalConfigException: Missing directory arguments.");
    if (this.directories.length === 0)
      throw new Error("FatalConfigException: Missing directory arguments.");

    if (!this.path)
      throw new Error(
        "FatalConfigException: Could not determine a path to an executable. This could be due to the host being an unknown or unsupported platform."
      );
    if (typeof this.path !== "string")
      throw new Error(
        "FatalConfigException: Could not determine a path to an executable. This could be due to the host being an unknown or unsupported platform."
      );
    if (this.path.length === 0)
      throw new Error(
        "FatalConfigException: Could not determine a path to an executable. This could be due to the host being an unknown or unsupported platform."
      );

    return true;
  };

  this.build = () => {
    if (this.validate()) {
      return new Executable(
        this.path,
        this.directories,
        this.port,
        this.cors,
        this.fallback,
        this.logLevel,
        this.noCompression,
        this.noPortSwitching
      );
    }
  };
}

export default ExecutableBuilder;
