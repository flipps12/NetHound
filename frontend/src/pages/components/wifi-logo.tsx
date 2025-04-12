export default function WifiLogo() {
    return (
      <div className="flex justify-center">
        <div className="relative w-20 h-20 flex items-center justify-center">
          <div className="absolute w-20 h-20 rounded-full bg-purple-900/30 animate-pulse"></div>
          <div
            className="absolute w-16 h-16 rounded-full bg-purple-800/20 animate-pulse"
            style={{ animationDelay: "300ms" }}
          ></div>
          <div
            className="absolute w-12 h-12 rounded-full bg-purple-700/10 animate-pulse"
            style={{ animationDelay: "600ms" }}
          ></div>
          <div className="relative">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="48"
              height="48"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              strokeWidth="2"
              strokeLinecap="round"
              strokeLinejoin="round"
              className="text-purple-400"
            >
              <path d="M5 12.55a11 11 0 0 1 14.08 0" />
              <path d="M1.42 9a16 16 0 0 1 21.16 0" />
              <path d="M8.53 16.11a6 6 0 0 1 6.95 0" />
              <line x1="12" y1="20" x2="12.01" y2="20" />
            </svg>
          </div>
        </div>
      </div>
    )
  }
  