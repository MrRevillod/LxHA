
import './index.css'

import ReactDOM from 'react-dom/client'

import { LandingPage } from './pages/Index'
import { ClientDashboardPage } from './pages/ClientDashboard'
import { BrowserRouter, Route, Routes, Link } from 'react-router-dom'
import { SessionProtectedRoute, RoleProtectedRoute } from './lib/router'
import { LoginPage } from './pages/auth/Login'
import { DashboardPage } from './pages/Dashboard'
import { useAuthStore } from './store/AuthStore'
import { ValidateAccountPage } from './pages/auth/ValidateAccount'

const root = ReactDOM.createRoot(document.getElementById('root')!)

root.render(

    <>
        <BrowserRouter>

            <Nav />

            <Routes>
                
                <Route path="/" element={<LandingPage />} />
                <Route path="/auth/login" element={<LoginPage />} />
                <Route path="/auth/validate-account/:id/:token" element={<ValidateAccountPage />} />

                <Route element={<SessionProtectedRoute />}>

                    <Route path="/dashboard" element={<ClientDashboardPage/>} />
                    
                </Route>

                <Route element={<RoleProtectedRoute />}>
                    <Route path="/admin/dashboard" element={<DashboardPage/>} />
                </Route>
                
            </Routes>
        </BrowserRouter>
    </>
)

function Nav() {

    const { useLogout } = useAuthStore()

    const handleLogout = async () => await useLogout()

    return (

        <>
            <nav className="flex flex-row w-full justify-between px-4">
                <div className="flex flex-row gap-4">
                    <Link to="/">Landing page</Link>
                    <Link to="/auth/login">Login page</Link>
                    <Link to="/dashboard">Client Dashboard</Link>
                    <Link to="/admin/dashboard">Admin Dashboard</Link>
                </div>
                <button onClick={() => handleLogout()}>Logout</button>
            </nav>
        </>
    )
}
