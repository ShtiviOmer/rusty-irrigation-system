import React from 'react';


const App = () => {
    const requestOptions = {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
    }
    const openValve = () => {
        fetch('/valve/open', requestOptions)
    }
    const closeValve = () => {
        fetch('/valve/close', requestOptions)
    }
    return (
        <div>
            <div><button onClick={openValve}>Open Valve</button></div>
            <div><button onClick={closeValve}>Close Valve</button></div>
        </div>
    );
}

export default App;