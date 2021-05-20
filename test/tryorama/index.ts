require = require('esm')(module)
import { InstallAgentsHapps, Orchestrator } from '@holochain/tryorama'
import { join } from 'path'
import { create_collective_get_collective_test } from './create_collective_get_collective.test'
// Point to your DNA file and give it a nickname.
// The DNA file can either be on your filesystem...
main().then()
async function main() {
  const protocol_love_dna = join(__dirname, '../../protocol.love.dna')
  const installation:InstallAgentsHapps = [
    // one agents
    [[protocol_love_dna]], // contains 1 dnaT
  ]
  process.on('unhandledRejection', error=>{
    // Will print "unhandledRejection err is not defined"
    console.log('unhandledRejection', error)
  })
  const orchestrator = new Orchestrator()
  create_collective_get_collective_test(orchestrator)
  const report = await orchestrator.run()
  console.log(report)
}
