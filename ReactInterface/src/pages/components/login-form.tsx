"use client"

import type React from "react"

import { useState } from "react"
import InputField from "./input-field"
import PasswordField from "./password-field.tsx"
import SubmitButton from "./submit-button.tsx"
import axios from 'axios';

export default function LoginForm() {
    const [username, setUsername] = useState("")
    const [password, setPassword] = useState("")
    const [loading, setLoading] = useState(false)

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault()
        setLoading(true)

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
                    setLoading(false)
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
        <form onSubmit={handleSubmit} className="space-y-4">
            <InputField
                label="Username"
                id="username"
                value={username}
                onChange={(e) => setUsername(e.target.value)}
                placeholder="Enter your username"
                required
            />

            <PasswordField
                label="Password"
                id="password"
                value={password}
                onChange={(e) => setPassword(e.target.value)}
                placeholder="Enter your password"
                required
            />

            {/* <div className="flex items-center justify-between">
                <div className="flex items-center">
                    <input
                        id="remember-me"
                        name="remember-me"
                        type="checkbox"
                        className="h-4 w-4 bg-gray-800 border-gray-700 text-purple-600 focus:ring-purple-500 rounded"
                    />
                    <label htmlFor="remember-me" className="ml-2 block text-sm text-gray-300">
                        Remember me
                    </label>
                </div>

                <div className="text-sm">
                    <a href="#" className="font-medium text-purple-400 hover:text-purple-300">
                        Need help?
                    </a>
                </div>
            </div> */} <br />

            <SubmitButton loading={loading} />

            <div className="text-center text-sm text-gray-400 mt-4">
                By logging in, you agree to our{" "}
                <a href="#" className="text-purple-400 hover:underline">
                    Terms of Service
                </a>
            </div>
        </form>
    )
}
