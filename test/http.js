require = require('esm')(module)
const test = require('tape-async')
const fetch = require('node-fetch')
const { TEST_URL } = process.env
const deepEqual = require('deep-equal')
const { assign, clone } = require('@ctx-core/object')
main()
async function main() {
	test('scenario: create_person, get_person, create_collective, get_collective, set_collective_name', async (t) => {
		const { person_hash, person } = await assert_create_person(t)
		await assert_get_person(t, { person_hash, person })
		const { collective_address, collective } =
			await assert_create_collective(t, { admin_hash: person_hash })
		await assert_get_collective_people(t,
			{ collective_address, collective_people: [person] })
		await assert_get_collective(t, { collective_address, collective })
		t.deepEqual(await _get_actions_result(t, collective_address), {
			Ok: {
				collective_address,
				actions: [
					_create_collective_action(collective),
					_set_collective_name_action(collective.name),
					_add_collective_person_action(person_hash),
				]
			}
		})
		const {
			collective: collective__renamed,
			collective_address: collective_address__renamed
		} =
			await assert_set_collective_name(t, {
				collective_address,
				collective,
				name: 'Renamed Collective'
			})
		t.equal(collective_address, collective_address__renamed)
		t.notEqual(collective.name, collective__renamed.name)
		await assert_get_collective(t, {
			collective_address: collective_address__renamed,
			collective: collective__renamed
		}, { timeout_ms: 5000 })
		t.deepEqual(await _get_actions_result(t, collective_address), {
			Ok: {
				collective_address: collective_address,
				actions: [
					_create_collective_action(collective),
					_set_collective_name_action(collective.name),
					_add_collective_person_action(person_hash),
					_set_collective_name_action(collective__renamed.name),
				]
			}
		})
	})
}
async function _api_result(t, params) {
	const response = await post_api(params)
	const json = await response.json()
	const { result } = json
	if (!result) {
		t.fail(JSON.stringify(json))
	}
	return JSON.parse(result)
}
async function post_api(params) {
	return fetch(TEST_URL, assign({
		method: 'POST',
		headers: {
			'Accept': 'application/json',
			'Content-Type': 'application/json',
		},
		body: JSON.stringify({
			id: '0',
			jsonrpc: '2.0',
			method: 'call',
			params
		})
	}))
}
function _api_params(function_name, args) {
	return {
		'instance_id': 'test-instance',
		'zome': 'protocol-love',
		'function': function_name,
		'args': args
	}
}
async function assert_create_person(t) {
	const create_person_result =
		await _api_result(t, _api_params(
			'create_person', {
				person: {
					name: 'Jane',
				}
			}
		))
	const { Ok } = create_person_result
	if (!Ok) {
		t.fail(JSON.stringify(create_person_result))
	}
	const { person_hash, person } = Ok
	t.assert(person_hash, 'person_hash should be truthy')
	const { agent_pubkey } = person
	t.assert(agent_pubkey)
	t.deepEqual(person, {
		agent_pubkey,
		name: 'Jane',
		status: 'Active',
	})
	return {
		person_hash,
		person,
	}
}
async function assert_get_person(t, { person_hash, person }, opts = {}) {
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
			await _api_result(t, _api_params(
				'get_person', {
					person_hash,
				}
			))
		const { Ok } = get_person_result
		if (!Ok) {
			t.fail(JSON.stringify(get_person_result))
		}
		return deepEqual(Ok, {
			person_hash,
			person,
		})
	}
}
async function assert_create_collective(t, { admin_hash }) {
	const create_collective_result =
		await _api_result(t, _api_params(
			'create_collective', {
				collective: {
					admin_hash,
					name: 'Flower of Life Collective',
				}
			}
		))
	const { Ok } = create_collective_result
	if (!Ok) {
		t.fail(JSON.stringify(create_collective_result))
	}
	const { collective_address, collective } = Ok
	t.assert(collective_address, 'collective_address should be truthy')
	t.deepEqual(collective, {
		name: 'Flower of Life Collective',
		admin_hash,
	})
	return {
		collective_address,
		collective,
	}
}
async function assert_get_collective_people(t, { collective_address, collective_people }) {
	const get_collective_people_result =
		await _api_result(t, _api_params(
			'get_collective_people', {
				collective_address,
			}
		))
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
async function assert_get_collective(t, { collective_address, collective }, opts = {}) {
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
			await _api_result(t, _api_params(
				'get_collective', {
					collective_address,
				}
			))
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
async function assert_set_collective_name(t, { collective_address, collective, name }) {
	t.notEqual(collective.name, name)
	const api_result = await _api_result(t, _api_params(
		'set_collective_name',
		{
			collective_address,
			name,
		}))
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
function _get_actions_result(t, collective_address) {
	return _api_result(t, _api_params(
		'get_actions',
		{
			collective_address
		}
	))
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
function _add_collective_person_action(person_hash) {
	return {
		op: 'AddCollectivePerson',
		status: 'Executed',
		data: JSON.stringify({ person_hash }),
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
