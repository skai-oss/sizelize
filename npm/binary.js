const { Binary } = require("binary-install");
const os = require("os");

const getPlatform = () => {
  const type = os.type();
  const arch = os.arch();

  if (arch === "x64") {
    switch (type) {
      case "Windows_NT":
        return "win64";
      case "Linux":
        return "linux";
      case "Darwin":
        return "macos";
    }
  }

  throw new Error(
    `Unsupported platform: ${type} ${arch}. Please create an issue at https://github.com/skai-oss/sizelize/issues`
  );
};

const getBinary = () => {
  const platform = getPlatform();
  const version = require("../package.json").version;
  const url = `https://github.com/skai-oss/sizelize/releases/download/v${version}/sizelize-${platform}.tar.gz`;
  return new Binary("sizelize", url);
};

const run = () => {
  const binary = getBinary();
  binary.run();
};

const install = () => {
  const binary = getBinary();
  binary.install();
};

module.exports = { install, run };
