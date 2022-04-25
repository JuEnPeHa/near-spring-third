import 'regenerator-runtime/runtime'
import React from 'react'
import { login, logout } from './utils'
import './global.css'
import { utils } from 'near-api-js'

import getConfig from './config'
const { networkId } = getConfig(process.env.NODE_ENV || 'testnet')

export default function App() {

  React.useEffect(
    () => {
      // in this case, we only care to query the contract when signed in
      if (window.walletConnection.isSignedIn()) {

      }
    },

    []
  )

  if (!window.walletConnection.isSignedIn()) {
    return (
      <main>
        <h1>Welcome to the the simplest NFT Minter on NEAR!</h1>
        <p style={{ textAlign: 'center', marginTop: '2.5em' }}>
          <button onClick={login}>Sign in</button>
        </p>
      </main>
    )
  }

  return (
    <>
      <button className="link" style={{ float: 'right' }} onClick={logout}>
        Sign out
      </button>
      <main>
        <h2>
          This is the simplest NFT Minter!
          {' '/* React trims whitespace around tags; insert literal space character when needed */}
          {window.accountId}!
        </h2>

              <button onClick={async event => {

                try {
                  await window.contract.nft_mint({

                  },
                  300000000000000, 
                  utils.format.parseNearAmount('1')
                 )
                } catch (e) {
                  alert(
                    'Something went wrong! ' +
                    'Maybe you need to sign out and back in? ' +
                    'Check your browser console for more info.'
                  )
                  throw e
                } finally {

                }
      
                setTimeout(() => {
                }, 11000)
              }}>MINT!</button>

        
      </main>
    </>
  )
}
