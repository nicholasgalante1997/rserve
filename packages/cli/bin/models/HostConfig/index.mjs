import os from "os";

function HostConfig() {
  this.arch = os.arch();
  this.cpus = os.cpus();
  this.machineType = os.machine();
  this.platform = os.platform();
}

export default HostConfig;
