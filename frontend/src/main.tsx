
import ReactDOM from "react-dom/client"

import "./index.css"
import "bootstrap-icons/font/bootstrap-icons.css"

import { Toaster } from "sonner"
import { MainLayout } from "./layouts/MainLayout"
import { AuthProvider } from "./store/AuthContext"
import { BrowserRouter, Routes, Route } from "react-router-dom"

import {
    LandingPage, LoginPage, ForgotPasswordPage, NotFoundPage,
    ForgotPasswordRequestPage, DashboardPage,
    AnalitycsPage,
    UsersPage,
    InstancesPage
} from "./pages"

import { LoadingWrapper, ProtectedRoute } from "./router"
import { SimpleLayout } from "./layouts/SimpleLayout"

const root = ReactDOM.createRoot(document.getElementById('root')!)

root.render(

    <>
        <AuthProvider>

            <BrowserRouter>

                <Routes>

                    <Route path="/" element={<LandingPage />} />

                    <Route element={<LoadingWrapper />}>
                        <Route path="/auth/login"
                            element={<SimpleLayout><LoginPage /></SimpleLayout>}
                        />
                        <Route path="/auth/reset-password"
                            element={<SimpleLayout><ForgotPasswordRequestPage /></SimpleLayout>}
                        />
                    </Route>

                    <Route path="/auth/reset-password/:id/:token"
                        element={<SimpleLayout> <ForgotPasswordPage /></SimpleLayout>}
                    />

                    <Route element={<ProtectedRoute protectedBy="session" />}>
                        <Route path="/dashboard" element={<MainLayout><DashboardPage /></MainLayout>} />
                        <Route path="/analitycs" element={<MainLayout><AnalitycsPage /></MainLayout>} />
                    </Route>

                    <Route element={<ProtectedRoute protectedBy="role" />}>
                        <Route path="/users" element={<MainLayout><UsersPage /></MainLayout>} />
                        <Route path="/instances" element={<MainLayout><InstancesPage /></MainLayout>} />
                    </Route>

                    <Route path="*" element={<SimpleLayout> <NotFoundPage /> </SimpleLayout>} />

                </Routes>

            </BrowserRouter>

        </AuthProvider>

        <Toaster />
    </>
)
