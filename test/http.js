const test = require('tape-async')
const fetch = require('node-fetch')
const { TEST_URL } = process.env
console.debug(`TEST_URL`, TEST_URL)
// Point to your DNA file and give it a nickname.
// The DNA file can either be on your filesystem...
main()
async function main() {
	test('scenario: create_collective, get_collective, set_collective_name', async (t) => {
		const { collective_address, collective } = await assert_create_collective(t)
		await assert_get_collective(t, { collective_address, collective })
		await assert_actions(t, { collective_address, collective })
	})
}
async function _api_result(params) {
	const response = await post_api(params)
	const json = await response.json()
	const { result } = json
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
async function assert_get_collective(t, { collective_address, collective }) {
	const get_collective_result =
		await _api_result(_api_params(
			'get_collective', {
				collective_address,
			}
		))
	t.deepEqual(get_collective_result, {
			Ok: {
				collective_address,
				collective,
			}
		}
	)
}
async function assert_actions(t, { collective_address, collective }) {
	const get_actions_result =
		await _api_result(_api_params(
			'get_actions',
			{
				collective_address
			}
		))
	t.deepEqual(get_actions_result, {
		Ok: {
			collective_address: collective_address,
			actions: [
				{
					op: 'CreateCollective',
					status: 'Executed',
					data: JSON.stringify(collective),
					tag: '',
					action_intent: 'SystemAutomatic'
				},
			]
		}
	})
}
function sleep(ms) {
	return new Promise(resolve => {
		setTimeout(resolve, ms)
	})
}
