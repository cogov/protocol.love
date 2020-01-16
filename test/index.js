const tryorama = require('@holochain/tryorama')
const path = require('path')
const fs = require('fs')
// Point to your DNA file and give it a nickname.
// The DNA file can either be on your filesystem...
main()
async function main() {
	const cogov_dna_path = path.join(__dirname, `../dist/${fs.readdirSync(`${__dirname}/../dist/`)}`)
	const cogov_dna = tryorama.Config.dna(cogov_dna_path, 'cogov')
	const main_config = tryorama.Config.gen({
		cogov: cogov_dna,
	}, {
		network: {
			type: 'sim2h',
			sim2h_url: 'ws://localhost:9000',
		},
	})
	const orchestrator = new tryorama.Orchestrator()
	orchestrator.registerScenario('create_collective; get_collective', async (s, t) => {
		const { alice } = await s.players({ alice: main_config, })
		await alice.spawn({})
//		const { carol } = await s.players({ carol: main_config }, true)
		const { collective_address, collective } = await assert_create_collective()
		await assert_actions({ collective_address, collective })
		async function assert_create_collective() {
			const create_collective_response = await alice.call('cogov', 'cogov', 'create_collective', {
				collective: {
					name: `Collective 1`
				}
			})
			const { Ok: { collective_address, collective } } = create_collective_response
			t.assert(collective_address, 'collective_address should be truthy')
			t.assert(collective, 'collective should be truthy')
			// TODO: figure out consistency in test
//		await s.consistency()
//		const get_collective_response = await carol.call('cogov', 'cogov', 'get_collective', {
			const get_collective_response = await alice.call('cogov', 'cogov', 'get_collective', {
				collective_address,
			})
			t.deepEqual(get_collective_response, {
					Ok: {
						collective_address,
						collective,
					}
				}
			)
			return {
				collective_address,
				collective,
			}
		}
		async function assert_actions({ collective_address, collective }) {
			const get_actions_response = await alice.call('cogov', 'cogov', 'get_actions', {
				collective_address,
			})
			t.deepEqual(get_actions_response, {
					Ok: [
						{
							collective_address,
							collective,
						},
					]
				}
			)
		}
	})
	const report = await orchestrator.run()
	console.log(report)
}
function sleep(ms) {
	return new Promise(resolve => {
		setTimeout(resolve, ms)
	})
}
