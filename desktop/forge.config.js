const { execSync } = require("child_process");
const path = require("path");

function hasBinary(command) {
  try {
    execSync(`command -v ${command}`, { stdio: "ignore" });
    return true;
  } catch {
    return false;
  }
}

/** @type {import("@electron-forge/shared-types").ForgeConfig} */
module.exports = {
  packagerConfig: {
    name: "Azure Portfolio",
    executableName: "azure-portfolio",
    appId: "dev.azuree0.portfolio",
    asar: true,
    extraResource: [path.join(__dirname, "..", "dist")],
  },
  makers: [
    {
      name: "@electron-forge/maker-squirrel",
      config: {
        name: "azure_portfolio",
      },
    },
    {
      name: "@electron-forge/maker-dmg",
      config: {},
    },
    {
      name: "@electron-forge/maker-deb",
      config: {
        options: {
          maintainer: "Azure",
          homepage: "https://azuree0-portfolio.pages.dev/",
        },
      },
    },
    {
      name: "@reforged/maker-appimage",
      config: {},
    },
    ...(hasBinary("snapcraft")
      ? [
          {
            name: "@electron-forge/maker-snap",
            config: {},
          },
        ]
      : []),
  ],
};
