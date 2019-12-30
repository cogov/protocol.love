"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const try_o_rama_1 = require("@holochain/try-o-rama");
const path_1 = require("path");
const fs_1 = require("fs");
// Point to your DNA file and give it a nickname.
// The DNA file can either be on your filesystem...
main();
async function main() {
    const cogov_dna_path = path_1.join(__dirname, `../dist/${fs_1.readdirSync(`${__dirname}/../dist/`)}`);
    const cogov_dna = try_o_rama_1.Config.dna(cogov_dna_path, 'cogov');
    // ... or on the web
    //const dnaChat = Config.dna('https://url.to/your/chat.dna.json', 'chat')
    // Set up a Conductor configuration using the handy `Conductor.config` helper.
    // Read the docs for more on configuration.
    const main_config = try_o_rama_1.Config.gen({
        cogov: cogov_dna,
    }, {
        // specify a bridge from chat to blog
        //		bridges: [Config.bridge('bridge-name', 'chat', 'blog')],
        // use a sim2h network (see conductor config options for all valid network types)
        network: {
            type: 'sim2h',
            sim2h_url: 'ws://localhost:9000',
        },
    });
    // Instatiate a test orchestrator.
    // It comes loaded with a lot default behavior which can be overridden, including:
    // * custom conductor spawning
    // * custom test result reporting
    // * scenario middleware, including integration with other test harnesses
    const orchestrator = new try_o_rama_1.Orchestrator();
    // Register a scenario, which is a function that gets a special API injected in
    orchestrator.registerScenario('commit_collective; get_collective', async (s, t) => {
        // Declare two players using the previously specified config,
        // and nickname them "alice" and "bob"
        const { alice } = await s.players({ alice: main_config, });
        // You have to spawn the conductors yourself...
        await alice.spawn({});
        // ...unless you pass `true` as an extra parameter,
        // in which case each conductor will auto-spawn
        const { carol } = await s.players({ carol: main_config }, true);
        // // You can also kill them...
        // await alice.kill()
        // // ...and re-spawn the same conductor you just killed
        // await alice.spawn({})
        // // now you can make zome calls,
        await alice.call('cogov', 'cogov', 'commit_collective', {
            name: `Collective 1`
        });
        // you can wait for total consistency of network activity,
        await s.consistency();
        // and you can make assertions using tape by default
        const messages = await carol.call('cogov', 'cogov', 'get_collective', {
            "collective": { "name": "Collective 0" },
        });
        t.equal(messages.length, 1);
    });
    // Run all registered scenarios as a final step, and gather the report,
    // if you set up a reporter
    const report = await orchestrator.run();
    // Note: by default, there will be no report
    console.log(report);
}
