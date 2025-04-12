import LoginForm from "./components/login-form.tsx"
import WifiLogo from "./components/wifi-logo.tsx"

function App() {
  

  return (
    <div className="min-h-screen bg-gradient-to-br from-black via-gray-900 to-gray-800 flex flex-col items-center justify-center p-4">
      <div className="w-full max-w-md bg-gray-900/80 backdrop-blur-sm rounded-xl shadow-2xl overflow-hidden border border-gray-800 relative">
        <div className="absolute inset-0 bg-gradient-to-br from-purple-500/5 to-transparent pointer-events-none"></div>
        <div className="absolute -inset-[1px] bg-gradient-to-br from-purple-500/20 via-transparent to-transparent rounded-xl pointer-events-none"></div>
        <div className="p-6 sm:p-8 relative z-10">
          <WifiLogo />
          <h1 className="text-2xl font-bold text-center text-white mt-4 mb-2">WiFi Hotspot</h1>
          <p className="text-center text-gray-400 mb-6">Please login to connect to the internet</p>
          <LoginForm />
        </div>
      </div>
    </div>
  );
}

export default App
