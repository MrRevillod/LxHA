
import "./index.css"
import ReactDOM from "react-dom/client"

import { Navbar } from "./components/Navbar"
import { Toaster } from "sonner"
import { LoginPage } from "./pages/auth/Login"
import { LandingPage } from "./pages/Index"
import { DashboardPage } from "./pages/Dashboard"
import { ResetPasswordPage } from "./pages/auth/ResetPassword"
import { ForgotPasswordPage } from "./pages/auth/ForgotPassword"
import { ClientDashboardPage } from "./pages/ClientDashboard"
import { BrowserRouter, Route, Routes } from "react-router-dom"
import { SessionProtectedRoute } from "./lib/router"

const root = ReactDOM.createRoot(document.getElementById('root')!)

root.render(

    <>
        <BrowserRouter>

            <Navbar />

            <Routes>

                <Route path="/" element={<LandingPage />} />
                <Route path="/auth/login" element={<LoginPage />} />
                <Route path="/auth/reset-password/" element={<ForgotPasswordPage />} />
                <Route path="/auth/reset-password/:id/:token" element={<ResetPasswordPage />} />

                <Route element={<SessionProtectedRoute />}>
                    <Route path="/dashboard" element={<ClientDashboardPage />} />
                </Route>

                <Route path="/admin/dashboard" element={<DashboardPage />} />

            </Routes>

            <Toaster richColors />

        </BrowserRouter>
    </>
)
