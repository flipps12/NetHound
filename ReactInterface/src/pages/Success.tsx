import React, { useEffect, useState } from "react";
import axios from "axios";

const Success: React.FC = () => {
    const [isOnline, setIsOnline] = useState<boolean | null>(null);

    useEffect(() => {
        const checkInternetConnection = async () => {
            try {
                await axios.get("https://www.google.com", { timeout: 5000 });
                setIsOnline(true);
            } catch (error) {
                setIsOnline(false);
            }
        };

        checkInternetConnection();
    }, []);

    return (
        <div className="flex flex-col items-center justify-center min-h-screen bg-green-100">
            <div className="bg-white p-6 rounded-lg shadow-md text-center">
                <h1 className="text-2xl font-bold text-green-600">¡Éxito!</h1>
                <p className="mt-2 text-gray-700">
                    La operación se completó correctamente.
                </p>
                {isOnline === null ? (
                    <p className="mt-4 text-blue-500">Comprobando conexión a internet...</p>
                ) : isOnline ? (
                    <p className="mt-4 text-green-500">Estás conectado a internet.</p>
                ) : (
                    <p className="mt-4 text-red-500">No tienes conexión a internet.</p>
                )}
            </div>
        </div>
    );
};

export default Success;