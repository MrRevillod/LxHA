
import { Link, useLocation } from "react-router-dom"
import { useAppStore } from "../store/AppStore"
import { useAuth } from "../store/AuthContext"
import { useUserStore } from "../store/UserStore"
import { Show } from "./ui/Show"
import { toUpperAndLower } from "../lib/string"

export const Header = () => {

    const { role } = useAuth()
    const { user } = useUserStore()
    const { pageTitle } = useAppStore()
    const { isAuthenticated } = useAuth()

    const location = useLocation()
    const isLoginPage = location.pathname === "/auth/login"

    return (

        <header className="w-full h-16 flex flex-row items-center justify-between text-neutral-950">

            <Show when={isAuthenticated}>
                <h1 className="text-2xl font-bold">{pageTitle}</h1>
                <div className="flex flex-col">
                    <h4 className="font-semibold">{user?.username}</h4>
                    <h5>{toUpperAndLower(role?.toString() || "")}</h5>
                </div>
            </Show>

            <Show when={!isAuthenticated}>
                <Link to="/"><i className="bi bi-box text-5xl"></i></Link>

                <Show when={isAuthenticated}>
                    <Link to="/dashboard" className="flex items-center justify-center text-lg w-36 h-12 px-4 rounded-md bg-primary text-white font-semibold">Dashboard</Link>
                </Show>

                <Show when={!isAuthenticated && !isLoginPage}>
                    <Link to="/auth/login" className="flex items-center justify-center text-lg w-36 h-12 px-4 rounded-md bg-primary text-white font-semibold">Login</Link>
                </Show>
            </Show>

        </header>
    )
}
