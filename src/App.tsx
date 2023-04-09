import { Event, listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/tauri';
import { createSignal } from 'solid-js';
import './App.css';

function App() {
  const [greetMsg, setGreetMsg] = createSignal('');

  // Listen for a file being dropped on the window to change the save location.
  listen('tauri://file-drop', async (event: Event<string[]>) => {
    if (event.payload.length > 0) {
      const file = event.payload[0];
      const result = await invoke('read_exif', { path: file });
      try {
        const data = JSON.parse(result);
        let str = '';
        for (const key of Object.keys(data)) {
          str += `${key}: ${JSON.stringify(JSON.parse(data[key]), null, 2)}\n`;
        }
        setGreetMsg(str);
      } catch (e) {
        setGreetMsg(result);
      }
    }
  });

  return (
    <div class='container'>
      <h1>Drag & Drop Image</h1>
      <pre>{greetMsg()}</pre>
    </div>
  );
}

export default App;
