import { useState } from 'react'
import axios from 'axios';

function App() {
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');

  const handleLogin = () => {
    console.log('Logging in with', { username, password });
    const queryParams = new URLSearchParams(window.location.search);
    const mac = queryParams.get('mac');
    const ip = queryParams.get('ip');

    console.log('Query parameters:', { mac, ip });
    axios.post('http://192.168.1.1/api/login', {
      username,
      password,
      //mac,
      ip
    })
      .then(response => {
        console.log('Login successful', response.data);
        if (response.data.verified) {
          console.log('Login successful');
          // Redirect to success page
          window.location.href = `http://192.168.1.1:8080/success?ip=${ip}`;
        } else {
          console.error('Login failed');
          alert('Login failed');
        }
      })
      .catch(error => {
        console.error('Login failed', error.response?.data || error.message);
      });
    console.log('Login request sent');
  };

  return (
    <div className='flex flex-col items-center justify-center min-h-screen bg-slate-800'>
      <div className='flex flex-col items-center justify-center w-11/12 bg-slate-900 p-8 rounded-2xl shadow-lg'>
        <h1 className='text-5xl font-semibold text-slate-500'>Login</h1>
        <div className='flex flex-col items-center justify-center mt-4 w-full'>
          <input
            className='text-white font-bold text-xl rounded-2xl bg-slate-700 outline-0 w-full mt-5 p-4'
            type="text"
            placeholder="Username"
            value={username}
            onChange={(e) => setUsername(e.target.value)}
          />
          <input
            className='text-white font-bold text-xl rounded-2xl bg-slate-700 outline-0 w-full mt-4 p-4'
            type="password"
            placeholder="Password"
            value={password}
            onChange={(e) => setPassword(e.target.value)}
          />
        </div>
        <button
          className='text-white font-semibold text-xl rounded-2xl bg-slate-950 outline-0 mt-6 w-full p-4'
          onClick={handleLogin}>
          Login
        </button>
      </div>
    </div>
  );
}

export default App
