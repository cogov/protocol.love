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
//		await s.consistency()
		const { person_address, person } = await assert_create_person(alice, t)
		await assert_get_person(alice, t, { person_address, person })
		const { collective_address, collective } = await assert_create_collective(alice, t, { person_address })
		// This next assertion times out
//		await assert_get_collective_creator(alice, t, { collective_address, collective_creator: person })
//		await assert_get_collective_people(alice, t, { collective_address, collective_people: [person] })
//		await assert_get_collective(alice, t, { collective_address, collective })
//		t.deepEqual(await _get_actions_result(alice, t, collective_address), {
//			Ok: {
//				collective_address: collective_address,
//				actions: [
//					_create_collective_action(collective),
//					_set_collective_name_action(collective.name),
//					_set_total_shares_action(collective.total_shares),
//					_add_collective_person_action(person_address),
//				]
//			}
//		})
//		const {
//			collective: collective__renamed,
//			collective_address: collective_address__renamed
//		} =
//			await assert_set_collective_name(t, {
//				collective_address,
//				collective,
//				name: 'Renamed Collective'
//			})
//		t.equal(collective_address, collective_address__renamed)
//		t.notEqual(collective.name, collective__renamed.name)
//		await assert_get_collective(alice, t, {
//			collective_address: collective_address__renamed,
//			collective: collective__renamed
//		}, { timeout_ms: 5000 })
//		t.deepEqual(await _get_actions_result(alice, t, collective_address), {
//			Ok: {
//				collective_address: collective_address,
//				actions: [
//					_create_collective_action(collective),
//					_set_collective_name_action(collective.name),
//					_set_total_shares_action(collective.total_shares),
//					_add_collective_person_action(person_address),
//					_set_collective_name_action(collective__renamed.name),
//				]
//			}
//		})
//		const {
//			collective: collective__total_shares,
//			collective_address: collective_address__total_shares
//		} =
//			await assert_set_collective_total_shares(alice, t, {
//				collective_address,
//				collective: collective__renamed,
//				total_shares: 750000
//			})
//		t.equal(collective_address, collective_address__total_shares)
//		t.notEqual(collective.total_shares, collective__total_shares.total_shares)
//		await assert_get_collective(t, {
//			collective_address: collective_address__total_shares,
//			collective: collective__total_shares
//		}, { timeout_ms: 5000 })
//		t.deepEqual(await _get_actions_result(alice, t, collective_address), {
//			Ok: {
//				collective_address: collective_address,
//				actions: [
//					_create_collective_action(collective),
//					_set_collective_name_action(collective.name),
//					_set_total_shares_action(collective.total_shares),
//					_add_collective_person_action(person_address),
//					_set_collective_name_action(collective__total_shares.name),
//					_set_total_shares_action(collective__total_shares.total_shares),
//				]
//			}
//		})
	})
	const report = await orchestrator.run()
	console.log(report)
}
async function player_call(player, name, params) {
	return player.call('cogov', 'cogov', name, params)
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
async function assert_create_collective(player, t, { person_address }) {
	const create_collective_result =
		await player_call(player,
			'create_collective', {
				collective: {
					person_address,
					name: 'Flower of Life Collective',
					total_shares: 500000
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
		total_shares: 500000
	})
	return {
		collective_address,
		collective,
	}
}
async function assert_get_collective_creator(player, t, { collective_address, collective_creator }) {
	const get_collective_creator_result =
		await player_call(player,
			'get_collective_creator', {
				collective_address,
			}
		)
	const { Ok } = get_collective_creator_result
	if (!Ok) {
		t.fail(JSON.stringify(get_collective_creator_result))
	}
	t.deepEqual(Ok, {
		collective_address,
		collective_creator,
	})
	return {
		collective_address,
		collective_creator,
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
async function assert_set_collective_total_shares(player, t, { collective_address, collective, total_shares }) {
	t.notEqual(collective.total_shares, total_shares)
	const api_result = await player_call(player,
		'set_collective_total_shares',
		{
			collective_address,
			total_shares,
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
		collective__result,
		clone(collective, { total_shares }))
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
		action_intent: 'SystemAutomatic'
	}
}
function _set_collective_name_action(name) {
	return {
		op: 'SetCollectiveName',
		status: 'Executed',
		data: JSON.stringify({ name }),
		tag: 'set_collective_name',
		action_intent: 'SystemAutomatic'
	}
}
function _set_total_shares_action(total_shares) {
	return {
		op: 'SetCollectiveTotalShares',
		status: 'Executed',
		data: JSON.stringify({ total_shares }),
		tag: 'set_collective_total_shares',
		action_intent: 'SystemAutomatic'
	}
}
function _add_collective_person_action(person_address) {
	return {
		op: 'AddCollectivePerson',
		status: 'Executed',
		data: JSON.stringify({ person_address }),
		tag: 'add_collective_person',
		action_intent: 'SystemAutomatic'
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
