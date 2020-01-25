const test = require('tape-async')
const fetch = require('node-fetch')
const { TEST_URL } = process.env
const deepEqual = require('deep-equal')
console.debug(`TEST_URL`, TEST_URL)
const assign = Object.assign
const clone = (...arg_a1) => assign({}, ...arg_a1)
main()
async function main() {
	test('scenario: create_collective, get_collective, set_collective_name', async (t) => {
		const { collective_address, collective } = await assert_create_collective(t)
		await assert_get_collective(t, { collective_address, collective })
		t.deepEqual(await _get_actions_result(collective_address), {
			Ok: {
				collective_address: collective_address,
				actions: [
					_create_collective_action(collective),
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
				collective_name: 'Renamed Collective'
			})
		t.equal(collective_address, collective_address__renamed)
		await assert_get_collective(t, {
			collective_address: collective_address__renamed,
			collective: collective__renamed
		}, { timeout_ms: 5000 })
	})
}
async function assert_set_collective_name(t, { collective_address, collective, collective_name }) {
	const api_result = await _api_result(_api_params(
		'set_collective_name',
		{
			collective_address,
			collective_name,
		}))
	const {
		Ok: {
			collective_address: collective_address__result,
			collective: collective__result,
		}
	} = api_result
	t.deepEqual(
		clone(collective, { name: collective_name }),
		collective__result)
	return {
		collective_address,
		collective: collective__result,
	}
}
async function _api_result(params) {
	const response = await post_api(params)
	const json = await response.json()
	const { result } = json
	if (!result) {
		throw json
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
		await _api_result(_api_params(
			'create_collective', {
				collective: {
					name: 'Collective 1'
				}
			}
		))
	console.debug('assert_create_collective|debug|1', {
		create_collective_response_json: create_collective_result,
	})
	const { Ok: { collective_address, collective } } = create_collective_result
	t.assert(collective_address, 'collective_address should be truthy')
	t.assert(collective, 'collective should be truthy')
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
			await _api_result(_api_params(
				'get_collective', {
					collective_address,
				}
			))
		return deepEqual(get_collective_result, {
				Ok: {
					collective_address,
					collective,
				}
			}
		)
	}
}
function _get_actions_result(collective_address) {
	return _api_result(_api_params(
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
		tag: '',
		action_intent: 'SystemAutomatic'
	}
}
async function wait_for(afn, timeout_ms = 5000, sleep_ms = 100) {
	const start_ms = _now_ms()
	console.debug('wait_for|debug|0', {
		start_ms,
		timeout_ms,
		sleep_ms,
	})
	while (!(await afn())) {
		console.debug('wait_for|debug|1', {
			now_ms: _now_ms(),
			'start_ms + timeout_ms': start_ms + timeout_ms,
		})
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
