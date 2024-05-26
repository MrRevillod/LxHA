
import { BrowserRouter, Routes, Route } from "react-router-dom"
import { LoadingWrapper, ProtectedRoute } from "./router"

import {
    ForgotPasswordRequestPage, DashboardPage,
    AnalitycsPage, UsersPage, InstancesPage,
    LandingPage, LoginPage, ForgotPasswordPage, NotFoundPage,
} from "./pages"

export const App = () => {

    return (

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
    )

}