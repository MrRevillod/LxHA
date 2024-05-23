
import ReactDOM from "react-dom/client"

import "./index.css"
import "bootstrap-icons/font/bootstrap-icons.css"

import { Toaster } from "sonner"
import { AuthProvider } from "./store/AuthContext"
import { BrowserRouter, Routes, Route } from "react-router-dom"
import { LoadingWrapper, ProtectedRoute } from "./router"

import {
    ForgotPasswordRequestPage, DashboardPage,
    AnalitycsPage, UsersPage, InstancesPage,
    LandingPage, LoginPage, ForgotPasswordPage, NotFoundPage,
} from "./pages"

const root = ReactDOM.createRoot(document.getElementById('root')!)

root.render(

    <>
        <AuthProvider>

            <BrowserRouter>

                <Routes>

                    <Route path="/" element={<LandingPage />} />

                    <Route element={<LoadingWrapper />}>
                        <Route path="/auth/login" element={<LoginPage />} />
                        <Route path="/auth/reset-password" element={<ForgotPasswordRequestPage />} />
                    </Route>

                    <Route path="/auth/reset-password/:id/:token" element={<ForgotPasswordPage />} />

                    <Route element={<ProtectedRoute protectedBy="session" />}>
                        <Route path="/dashboard" element={<DashboardPage />} />
                        <Route path="/analitycs" element={<AnalitycsPage />} />
                    </Route>

                    <Route element={<ProtectedRoute protectedBy="role" />}>
                        <Route path="/users" element={<UsersPage />} />
                        <Route path="/instances" element={<InstancesPage />} />
                    </Route>

                    <Route path="*" element={<NotFoundPage />} />

                </Routes>

            </BrowserRouter>

        </AuthProvider>

        <Toaster />
    </>
)
