import { useCallback, useRef } from 'react'
import { invoke } from '@tauri-apps/api'
import './App.css'

function App () {
  const interval = useRef()

  const changeTrayIcon = useCallback(async (iconType) => {
    try {
      if (interval.current) clearInterval(interval.current)
      if (iconType === 'default') {
        await invoke('change_tray_icon', { iconType, iconIndex: 0 })
      } else if (iconType === 'loading') {
        let i = 0
        interval.current = setInterval(async () => {
          await invoke('change_tray_icon', { iconType, iconIndex: i })
          if (i >= 11) {
            i = 0
          } else {
            i += 1
          }
        }, 30)
      }
    } catch (e) {
      console.log(e)
    }
  }, [])

  return (
    <div className="App">
      <button onClick={() => changeTrayIcon('default')}>default tray icon</button>

      <button onClick={() => changeTrayIcon('loading')}>loading tray icon</button>
    </div>
  )
}

export default App
