
import { Link, useLocation } from "react-router-dom"
import { useAppStore } from "../store/AppStore"
import { useAuth } from "../store/AuthContext"
import { useUserStore } from "../store/UserStore"
import { Show } from "./ui/Show"

export const Header = () => {

    const { user } = useUserStore()
    const { pageTitle } = useAppStore()

    return (

        <header className="w-full h-16 flex flex-row items-center justify-between text-neutral-950">
            <h1 className="text-2xl font-bold">{pageTitle}</h1>
            <h4>{user?.username}</h4>
        </header>
    )
}

export const LandingHeader = () => {

    const { user } = useUserStore()
    const { isAuthenticated } = useAuth()

    const location = useLocation()
    const isLoginPage = location.pathname === "/auth/login"

    return (

        <header className="w-full h-16 flex flex-row items-center justify-between text-neutral-950">

            <i className="bi bi-box text-5xl"></i>

            <Show when={isAuthenticated}>
                <h4>{user?.username}</h4>
            </Show>

            <Show when={!isAuthenticated && !isLoginPage}>
                <Link to="/auth/login" className="flex items-center justify-center text-lg w-36 h-12 px-4 rounded-md bg-primary text-white font-semibold">Login</Link>
            </Show>

        </header>
    )
}