
import ReactDOM from "react-dom/client"

import "./index.css"
import "bootstrap-icons/font/bootstrap-icons.css"

import { Toaster } from "sonner"
import { MainLayout } from "./layouts/MainLayout"
import { AuthProvider } from "./store/AuthContext"
import { BrowserRouter, Routes, Route } from "react-router-dom"

import {
    LandingPage, LoginPage, ForgotPasswordPage, NotFoundPage,
    ForgotPasswordRequestPage, DashboardPage
} from "./pages"

import { LoadingWrapper, ProtectedRoute } from "./router"

const root = ReactDOM.createRoot(document.getElementById('root')!)

root.render(

    <>
        <AuthProvider>
            <BrowserRouter>
                <Routes>

                    <Route path="/" element={<MainLayout><LandingPage /></MainLayout>} />

                    <Route element={<LoadingWrapper />}>
                        <Route path="/auth/login" element={<LoginPage />} />
                        <Route path="/auth/reset-password" element={<MainLayout><ForgotPasswordRequestPage /></MainLayout>} />
                    </Route>

                    <Route path="/auth/reset-password/:id/:token" element={<MainLayout> <ForgotPasswordPage /></MainLayout>} />

                    <Route element={<ProtectedRoute protectedBy="session" />}>
                        <Route path="/dashboard" element={<MainLayout><DashboardPage /></MainLayout>} />
                    </Route>

                    <Route path="*" element={<MainLayout> <NotFoundPage /> </MainLayout>} />

                </Routes>
            </BrowserRouter>
        </AuthProvider>

        <Toaster />
    </>
)
