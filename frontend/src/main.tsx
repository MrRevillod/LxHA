
import ReactDOM from "react-dom/client"

import "./index.css"
import "bootstrap-icons/font/bootstrap-icons.css"

import { App } from "./app"
import { Toaster } from "sonner"
import { AuthProvider } from "./store/AuthContext"

const root = ReactDOM.createRoot(document.getElementById('root')!)

root.render(

    <>
        <AuthProvider>

            <App />

        </AuthProvider>

        <Toaster />
    </>
)
