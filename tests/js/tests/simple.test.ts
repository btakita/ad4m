import { expect } from "chai";
import { ChildProcess } from 'node:child_process';
import { Ad4mClient } from "@perspect3vism/ad4m";
import { startExecutor, apolloClient, sleep } from "../utils/utils";
import path from "path";
import fetch from 'node-fetch'
import { fileURLToPath } from 'url';

//@ts-ignore
global.fetch = fetch

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

describe("Integration", () => {
  const TEST_DIR = path.join(`${__dirname}/../tst-tmp`);
  const appDataPath = path.join(TEST_DIR, "agents", "alice");
  const bootstrapSeedPath = path.join(`${__dirname}/../bootstrapSeed.json`);
  const gqlPort = 15000
  const hcAdminPort = 15001
  const hcAppPort = 15002
  const ipfsSwarmPort = 15007

  let ad4m: Ad4mClient | null = null
  let executorProcess: ChildProcess | null = null

  before(async () => {
    executorProcess = await startExecutor(appDataPath, bootstrapSeedPath,
        gqlPort, hcAdminPort, hcAppPort, ipfsSwarmPort);

    console.log("Creating ad4m client")
    ad4m = new Ad4mClient(apolloClient(gqlPort))
    console.log("Generating agent")
    await ad4m.agent.generate("secret")
    console.log("Done")
  })

  after(async () => {
    while (!executorProcess?.killed) {
      let status  = executorProcess?.kill();
      console.log("killed executor with", status);
      await sleep(500);
    }
  })

  it("should get agent status", async () => {
    let result = await ad4m!.agent.status()
    expect(result).to.not.be.null
    expect(result!.isInitialized).to.be.true
  })
})