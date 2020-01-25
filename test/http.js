const test = require('tape-async')
const fetch = require('node-fetch')
const { TEST_URL } = process.env
const deepEqual = require('deep-equal')
console.debug(`TEST_URL`, TEST_URL)
const assign = Object.assign
const clone = (...arg_a1) => assign({}, ...arg_a1)
main()
async function main() {
	test('scenario: create_collective, get_collective, set_collective_name, set_collective_total_shares', async (t) => {
		const { collective_address, collective } = await assert_create_collective(t)
		await assert_get_collective(t, { collective_address, collective })
		t.deepEqual(await _get_actions_result(t, collective_address), {
			Ok: {
				collective_address: collective_address,
				actions: [
					_create_collective_action(collective),
					_set_collective_name_action(collective.name),
					_set_total_shares_action(collective.total_shares),
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
					_set_total_shares_action(collective.total_shares),
					_set_collective_name_action(collective__renamed.name),
				]
			}
		})
		const {
			collective: collective__total_shares,
			collective_address: collective_address__total_shares
		} =
			await assert_set_collective_total_shares(t, {
				collective_address,
				collective: collective__renamed,
				total_shares: 750000
			})
		t.equal(collective_address, collective_address__total_shares)
		t.notEqual(collective.total_shares, collective__total_shares.total_shares)
		await assert_get_collective(t, {
			collective_address: collective_address__total_shares,
			collective: collective__total_shares
		}, { timeout_ms: 5000 })
		t.deepEqual(await _get_actions_result(t, collective_address), {
			Ok: {
				collective_address: collective_address,
				actions: [
					_create_collective_action(collective),
					_set_collective_name_action(collective.name),
					_set_total_shares_action(collective.total_shares),
					_set_collective_name_action(collective__total_shares.name),
					_set_total_shares_action(collective__total_shares.total_shares),
				]
			}
		})
	})
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
async function assert_set_collective_total_shares(t, { collective_address, collective, total_shares }) {
	t.notEqual(collective.total_shares, total_shares)
	const api_result = await _api_result(t, _api_params(
		'set_collective_total_shares',
		{
			collective_address,
			total_shares,
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
		collective__result,
		clone(collective, { total_shares }))
	return {
		collective_address,
		collective: collective__result,
	}
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
	return fetch(TEST_URL, Object.assign({
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
		'zome': 'cogov',
		'function': function_name,
		'args': args
	}
}
async function assert_create_collective(t) {
	const create_collective_result =
		await _api_result(t, _api_params(
			'create_collective', {
				collective: {
					name: 'Flower of Life Collective',
					total_shares: 500000
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
		total_shares: 500000
	})
	return {
		collective_address,
		collective,
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
