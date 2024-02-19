import { useState } from 'react';
import './App.css';

function App() {
  const [message, setMessage] = useState('')
  async function getMessage() {
    await fetch('http://127.0.0.1:3030')
      .then(res => res.json()).then(data => setMessage(data.message))
  }
  return (
    <div className="App">
      <header className="App-header">
        <div>{message}</div>
        <button onClick={getMessage}>
          Click!
        </button>
      </header>
    </div>
  );
}

export default App;
