
export const reinstall_all = async (options?: CanisterReinstallOptions) => {
    // recode time of cost
    const start = Date.now();
    const jobs = Array<Promise<void>>();


    const get_jobs = function* () {
        yield step1();

        function* step1() {
        }

    };

    if (options && options.one_by_one) {
        for (const job_step of get_jobs()) {
            for (const job of job_step) {
                await job;
            }
        }
    } else {
        console.info("reinstall all in parallel");
        for (const job_step of get_jobs()) {
            await Promise.all(job_step);
        }
    }


    const end = Date.now();
    console.info(`reinstall all in ${end - start} ms`);
    // sleep for 3 seconds to waiting code to be available
    await new Promise((resolve) => setTimeout(resolve, 3000));
}

export interface Fee {
    minimum: number,
    rate: number,
    rate_decimals: number
}

export interface YourCanisterInitOptions {
    name: string;
}

export interface DFTInitOptions {
    name: string;
    symbol: string;
    decimals: bigint;
    totalSupply: bigint;
    fee?: Fee;
    desc?: [string, string][];
    owner: string;
    archive?: number;
    threshold?: number;
}

export interface CommonInstallOptions {
    reinstall: boolean;
}

export interface DFTInstallOptions extends CommonInstallOptions {
    initOptions?: DFTInitOptions;
}

export interface CanisterReinstallOptionsCanisters {
    dft?: DFTInstallOptions;
}

export interface CanisterReinstallOptions {
    build?: boolean;
    init?: boolean;
    one_by_one?: boolean;
    canisters?: CanisterReinstallOptionsCanisters;
}
