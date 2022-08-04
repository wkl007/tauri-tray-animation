import { useCallback } from 'react'
import { invoke } from '@tauri-apps/api'
import './App.css'

function App () {
  const changeTrayIcon = useCallback((iconType) => {
    invoke('change_tray_icon', { iconType })
  }, [])

  return (
    <div className="App">
      <button onClick={() => changeTrayIcon('default')}>default tray icon</button>

      <button onClick={() => changeTrayIcon('loading')}>loading tray icon</button>
    </div>
  )
}

export default App
