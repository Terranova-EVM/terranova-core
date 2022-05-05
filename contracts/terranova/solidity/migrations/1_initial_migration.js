const Migrations = artifacts.require("Migrations");
const Storage = artifacts.require("Storage");

module.exports = function (deployer) {
  deployer.deploy(Storage);
  deployer.deploy(Migrations);
};
