import * as fs from "fs";
import { identity, dfxJson, canister } from "@deland-labs/ic-dev-kit";

(async () => {
  await canister.createAll();
  const names = dfxJson.get_dfx_json().canisters.keys();
  const dir = "./env_configs";
  // create dir if not exists
  if (!fs.existsSync(dir)) {
    fs.mkdirSync(dir, { recursive: true });
  }

  let env_canister_ids_content = fs
    .readFileSync("./env_configs/dev.canister_ids.env")
    .toString();
  for (const name of names) {
    const env_name = `COMMON_CANISTER_IDS_${name.toUpperCase()}`;
    const value = canister.get_id(name);
    const config_value = `export ${env_name}=${value}\n`;
    if (env_canister_ids_content.includes(env_name)) {
      env_canister_ids_content = env_canister_ids_content.replace(
        new RegExp(`^export ${env_name}.*$`, "gm"),
        config_value
      );
    } else {
      env_canister_ids_content += config_value;
    }
  }

  // write env file
  fs.writeFileSync(`${dir}/dev.canister_ids.env`, env_canister_ids_content);

  const admin = `COMMON_PRINCIPAL_NAME_ADMIN`;
  const admin_v = identity.identityFactory.getPrincipal("dev_main")?.toText();

  const principalContent = `export ${admin}="
# main node
${admin_v}
"
`;
  fs.writeFileSync(`${dir}/dev.principals.env`, principalContent);
})();
