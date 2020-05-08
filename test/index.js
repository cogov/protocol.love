require = require('esm')(module)
const tryorama = require('@holochain/tryorama')
const path = require('path')
const fs = require('fs')
const deepEqual = require('deep-equal')
const { clone } = require('@ctx-core/object')
// Point to your DNA file and give it a nickname.
// The DNA file can either be on your filesystem...
main()
async function main() {
	const protocol_love_dna_path = path.join(__dirname, `../dist/${fs.readdirSync(`${__dirname}/../dist/`)}`)
	const protocol_love_dna = tryorama.Config.dna(protocol_love_dna_path, 'protocol-love')
	const main_config = tryorama.Config.gen({
		'protocol-love': protocol_love_dna,
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
		const { carol } = await s.players({ carol: main_config }, true)
		await s.consistency()
		const { person_address, person } = await assert_create_person(alice, t)
		await assert_get_person(alice, t, { person_address, person })
		const { collective_address, collective } =
			await assert_create_collective(alice, t, {
				admin_address: person_address,
			})
		await assert_get_collective_people(alice, t, {
			collective_address,
			collective_people: [person],
		})
		await assert_get_collective(alice, t, {
			collective_address,
			collective,
		})
		t.deepEqual(
			await _get_actions_result(alice, t, collective_address),
			{
				Ok: {
					collective_address: collective_address,
					actions: [
						_create_collective_action(collective),
						_set_collective_name_action(collective.name),
						_add_collective_person_action(person_address),
					]
				}
			})
		const {
			collective: collective__renamed,
			collective_address: collective_address__renamed
		} =
			await assert_set_collective_name(alice, t, {
				collective_address,
				collective,
				name: 'Renamed Collective'
			})
		t.equal(collective_address, collective_address__renamed)
		t.notEqual(collective.name, collective__renamed.name)
		await assert_get_collective(alice, t, {
			collective_address: collective_address__renamed,
			collective: collective__renamed
		}, { timeout_ms: 5000 })
		t.deepEqual(await _get_actions_result(alice, t, collective_address),
			{
				Ok: {
					collective_address: collective_address,
					actions: [
						_create_collective_action(collective),
						_set_collective_name_action(collective.name),
						_add_collective_person_action(person_address),
						_set_collective_name_action(collective__renamed.name),
					]
				}
			})
		t.deepEqual(await _get_actions_result(alice, t, collective_address), {
			Ok: {
				collective_address: collective_address,
				actions: [
					_create_collective_action(collective),
					_set_collective_name_action(collective.name),
					_add_collective_person_action(person_address),
					_set_collective_name_action(collective__renamed.name),
				]
			}
		})
	})
	const report = await orchestrator.run()
	console.log(report)
}
async function player_call(player, name, params) {
	return player.call('protocol-love', 'protocol-love', name, params)
}
async function assert_create_person(player, t) {
	const create_person_result =
		await player_call(player, 'create_person', {
				person: {
					name: 'Jane',
				}
			}
		)
	const { Ok } = create_person_result
	if (!Ok) {
		t.fail(JSON.stringify(create_person_result))
	}
	const { person_address, person } = Ok
	t.assert(person_address, 'person_address should be truthy')
	const { agent_address } = person
	t.assert(agent_address)
	t.deepEqual(person, {
		agent_address,
		name: 'Jane',
		status: 'Active',
	})
	return {
		person_address,
		person,
	}
}
async function assert_get_person(player, t, { person_address, person }, opts = {}) {
	const { timeout_ms } = opts
	if (timeout_ms != null) {
		await wait_for(
			async () =>
				do_assert_get_person(deepEqual),
			timeout_ms)
	}
	return do_assert_get_person(t.deepEqual)
	async function do_assert_get_person(deepEqual) {
		const get_person_result =
			await player_call(player, 'get_person', { person_address })
		const { Ok } = get_person_result
		if (!Ok) {
			t.fail(JSON.stringify(get_person_result))
		}
		return deepEqual(Ok, {
			person_address,
			person,
		})
	}
}
async function assert_create_collective(player, t, { admin_address }) {
	const create_collective_result =
		await player_call(player,
			'create_collective', {
				collective: {
					admin_address,
					name: 'Flower of Life Collective',
				}
			}
		)
	const { Ok } = create_collective_result
	if (!Ok) {
		t.fail(JSON.stringify(create_collective_result))
	}
	const { collective_address, collective } = Ok
	t.assert(collective_address, 'collective_address should be truthy')
	t.deepEqual(collective, {
		name: 'Flower of Life Collective',
		admin_address,
	})
	return {
		collective_address,
		collective,
	}
}
async function assert_get_collective_people(player, t, { collective_address, collective_people }) {
	const get_collective_people_result =
		await player_call(player,
			'get_collective_people', {
				collective_address,
			}
		)
	const { Ok } = get_collective_people_result
	if (!Ok) {
		t.fail(JSON.stringify(get_collective_people_result))
	}
	t.deepEqual(Ok, {
		collective_address,
		collective_people,
	})
	return {
		collective_address,
		collective_people,
	}
}
async function assert_get_collective(player, t, { collective_address, collective }, opts = {}) {
	const { timeout_ms } = opts
	if (timeout_ms != null) {
		await wait_for(
			async () =>
				do_assert_get_collective(deepEqual),
			timeout_ms)
	}
	return do_assert_get_collective(t.deepEqual)
	async function do_assert_get_collective(deepEqual) {
		const get_collective_result =
			await player_call(player,
				'get_collective', {
					collective_address,
				}
			)
		const { Ok } = get_collective_result
		if (!Ok) {
			t.fail(JSON.stringify(get_collective_result))
		}
		return deepEqual(Ok, {
			collective_address,
			collective,
		})
	}
}
async function assert_set_collective_name(player, t, { collective_address, collective, name }) {
	t.notEqual(collective.name, name)
	const api_result = await player_call(player,
		'set_collective_name', {
			collective_address,
			name,
		}
	)
	const { Ok } = api_result
	if (!Ok) {
		t.fail(JSON.stringify(api_result))
	}
	const {
		collective_address: collective_address__result,
		collective: collective__result,
	} = Ok
	t.deepEqual(
		clone(collective, { name }),
		collective__result)
	return {
		collective_address,
		collective: collective__result,
	}
}
function _get_actions_result(player, t, collective_address) {
	return player_call(player,
		'get_actions',
		{
			collective_address
		}
	)
}
function _create_collective_action(collective) {
	return {
		op: 'CreateCollective',
		status: 'Executed',
		data: JSON.stringify(collective),
		tag: 'create_collective',
		strategy: 'SystemAutomatic'
	}
}
function _set_collective_name_action(name) {
	return {
		op: 'SetCollectiveName',
		status: 'Executed',
		data: JSON.stringify({ name }),
		tag: 'set_collective_name',
		strategy: 'SystemAutomatic'
	}
}
function _add_collective_person_action(person_address) {
	return {
		op: 'AddCollectivePerson',
		status: 'Executed',
		data: JSON.stringify({ person_address }),
		tag: 'add_collective_person',
		strategy: 'SystemAutomatic'
	}
}
async function wait_for(afn, timeout_ms = 5000, sleep_ms = 100) {
	const start_ms = _now_ms()
	while (!(await afn())) {
		if (_now_ms() > (start_ms + timeout_ms)) {
			return false
		}
		await sleep(sleep_ms)
	}
	return true
}
function _now_ms() {
	return new Date().getTime()
}
function sleep(ms) {
	return new Promise(resolve => {
		setTimeout(resolve, ms)
	})
}
