import { useEffect, useState } from 'react'
import reactLogo from './assets/react.svg'
import viteLogo from '/vite.svg'
import './App.css'
import * as sim from './wasm'
import SimCanvas from './components/SimCanvas'

function App() {
  const [count, setCount] = useState(0);
  const [msg, setMsg] = useState<string>('Loading WASM…')

  useEffect(() => {
    sim.wasmReady.then(() => {
      setMsg(sim.test_fun())
    })
  }, [])

  return (
    <>
      <div>
        <SimCanvas />
      </div>
    </>
  )
}

export default App
