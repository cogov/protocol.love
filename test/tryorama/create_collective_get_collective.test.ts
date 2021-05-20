import { join } from 'path'
require = require('esm')(module)
import { Config, InstallAgentsHapps } from '@holochain/tryorama'
import deepEqual from 'deep-equal'
import { clone } from '@ctx-core/object'
import { timeout_opts_I } from '../common'
export function create_collective_get_collective_test(orchestrator) {
  const config = Config.gen()
  orchestrator.registerScenario('create_collective; get_collective', async (s, t)=>{
    const dna = join(__dirname, '../../protocol.love.dna')
    const installation:InstallAgentsHapps = [
      // one agents
      [[dna]], // contains 1 dnaT
    ]
    const [me_player, alice_player, bob_player] = await s.players([config, config, config])
    const [[me_happ]] = await me_player.installAgentsHapps(installation)
    const [[alice_happ]] = await alice_player.installAgentsHapps(installation)
    const [[bob_happ]] = await bob_player.installAgentsHapps(installation)
    await s.shareAllNodes([me_player, alice_player, bob_player])

    const me = me_happ.cells[0]
    const alice = alice_happ.cells[0]
    const bob = bob_happ.cells[0]
    const me_pubkey = me.cellId[1]
    const alice_pubkey = alice.cellId[1]
    const alice_pubkey_base64 = alice_pubkey.toString('base64')
    const bob_pubkey = bob.cellId[1]
    const bob_pubkey_base64 = bob_pubkey.toString('base64')

    const { person_entry_hash, person } = await assert_create_person(alice, t)
    await assert_get_person(alice, t, { person_entry_hash, person })
    const { collective_address, collective } =
      await assert_create_collective(alice, t, {
        admin_entry_hash: person_entry_hash,
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
            _set_collective_name_action(collective.name, null),
            _add_collective_person_action(person_entry_hash),
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
            _set_collective_name_action(collective.name, null),
            _add_collective_person_action(person_entry_hash),
            _set_collective_name_action(collective__renamed.name, collective.name),
          ]
        }
      })
  })
}
async function player_call(player, name:string, params) {
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
  const { person_entry_hash, person } = Ok
  t.assert(person_entry_hash, 'person_entry_hash should be truthy')
  const { agent_pub_key } = person
  t.assert(agent_pub_key)
  t.deepEqual(person, {
    agent_pub_key,
    name: 'Jane',
    status: 'Active',
  })
  return {
    person_entry_hash,
    person,
  }
}
async function assert_get_person(
  player,
  t,
  { person_entry_hash, person },
  opts:timeout_opts_I = {}
) {
  const { timeout_ms } = opts
  if (timeout_ms != null) {
    await wait_for(
      async ()=>
        do_assert_get_person(deepEqual),
      timeout_ms)
  }
  return do_assert_get_person(t.deepEqual)
  async function do_assert_get_person(deepEqual) {
    const get_person_result =
      await player_call(player, 'get_person', { person_entry_hash })
    const { Ok } = get_person_result
    if (!Ok) {
      t.fail(JSON.stringify(get_person_result))
    }
    return deepEqual(Ok, {
      person_entry_hash,
      person,
    })
  }
}
async function assert_create_collective(player, t, { admin_entry_hash }) {
  const create_collective_result =
    await player_call(player,
      'create_collective', {
        collective: {
          admin_entry_hash,
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
    admin_entry_hash,
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
async function assert_get_collective(
  player, t, { collective_address, collective }, opts:timeout_opts_I = {}
) {
  const { timeout_ms } = opts
  if (timeout_ms != null) {
    await wait_for(
      async ()=>
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
async function assert_set_collective_name(
  player, t, { collective_address, collective, name }
) {
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
    prev_data: JSON.stringify(null),
    tag: 'create_collective',
    strategy: 'SystemAutomatic'
  }
}
function _set_collective_name_action(name, prev_name) {
  return {
    op: 'SetCollectiveName',
    status: 'Executed',
    data: JSON.stringify({ name }),
    prev_data: JSON.stringify(prev_name && { name: prev_name }),
    tag: 'set_collective_name',
    strategy: 'SystemAutomatic'
  }
}
function _add_collective_person_action(person_entry_hash) {
  return {
    op: 'AddCollectivePerson',
    status: 'Executed',
    data: JSON.stringify({ person_entry_hash }),
    prev_data: JSON.stringify(null),
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
function sleep(ms:number) {
  return new Promise(resolve=>{
    setTimeout(resolve, ms)
  })
}
