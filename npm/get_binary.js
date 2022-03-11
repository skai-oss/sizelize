const { Binary } = require("binary-install");
const os = require("os");

function getPlatform() {
  const type = os.type();
  const arch = os.arch();

  if (type === "Windows_NT") {
    if (arch === "x64") {
      return "win64";
    }
  }

  if (type === "Linux" && arch === "x64") {
    return "linux";
  }

  if (type === "Darwin" && arch === "x64") {
    return "macos";
  }

  throw new Error(
    `Unsupported platform: ${type} ${arch}. Please create an issue at https://github.com/skai-oss/sizelize/issues`
  );
}

function getBinary() {
  const platform = getPlatform();
  const version = require("../package.json").version;
  const url = `https://github.com/skai-oss/sizelize/releases/download/v${version}/sweep-${platform}.tar.gz`;
  return new Binary(url, { name: "sizelize" });
}

module.exports = getBinary;
