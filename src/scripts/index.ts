import { Argument, Command, OptionValues } from "commander";

const program = new Command();

program
    .command("reinstall-all")
    .description("reinstall all canisters")
    .action(async () => {
        require("./reinstall_all");
    });

program
    .command("update-local-config")
    .description("update local config")
    .action(async () => {
        require("./updateLocalConfigs");
    });

program.parse(process.argv);
