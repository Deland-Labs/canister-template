import * as fs from 'fs'
import {identity, dfxJson, canister} from '@deland-labs/ic-dev-kit';

(async () => {

    await canister.createAll();
    const names = dfxJson.get_dfx_json().canisters.keys()
    const dir = './env_configs'
    // create dir if not exists
    if (!fs.existsSync(dir)) {
        fs.mkdirSync(dir, {recursive: true})
    }

    let env_file_content = "";
    for (const name of names) {
        const env_name = `COMMON_CANISTER_IDS_${name.toUpperCase()}`;
        const value = canister.get_id(name);
        env_file_content += `export ${env_name}=${value}\n`;
    }
    // write env file
    fs.writeFileSync(`${dir}/dev.canister_ids.env`, env_file_content);

    const admin = `COMMON_PRINCIPAL_NAME_ADMIN`;
    const admin_v = identity.identityFactory.getPrincipal('dev_main')?.toText();


    const principalContent = `export ${admin}="
# main node
${admin_v}
"
`
    fs.writeFileSync(`${dir}/dev.principals.env`, principalContent);


})()
