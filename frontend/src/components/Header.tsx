
import { Show } from "./ui/Show"
import { useAuth } from "../store/AuthContext"
import { useAppStore } from "../store/AppStore"
import { toUpperAndLower } from "../lib/string"
import { Link, useLocation } from "react-router-dom"

export const Header = () => {

    const { user, role } = useAuth()
    const { pageTitle } = useAppStore()
    const { isAuthenticated } = useAuth()

    const location = useLocation()
    const isLoginPage = location.pathname === "/auth/login"
    const isRoot = location.pathname === "/"

    return (

        <header className="w-full h-16 flex flex-row items-center justify-between text-neutral-950 pt-10">

            <Show when={isAuthenticated}>

                <Show when={isRoot}>
                    <div className="flex flex-row gap-8 items-center">
                        <Link to="/"><i className="bi bi-box text-5xl"></i></Link>
                        <h1 className="text-2xl font-bold">{`Welcome again ${user?.name}!`}</h1>
                    </div>
                </Show>

                <Show when={!isRoot}>
                    <h1 className="text-3xl font-bold">{pageTitle}</h1>
                </Show>

                <div className="flex flex-row gap-12">

                    <Show when={isRoot}>
                        <Link to="/dashboard" className="flex items-center justify-center text-lg w-36 h-12 px-4 rounded-md border-2 border-primary text-primary font-semibold">
                            Dashboard
                        </Link>
                    </Show>

                    <Show when={!isRoot}>
                        <div className="flex flex-col">
                            <h4 className="font-semibold">{user?.name}</h4>
                            <h5>{toUpperAndLower(role?.toString() || "")}</h5>
                        </div>
                    </Show>

                </div>

            </Show>

            <Show when={!isAuthenticated}>

                <Link to="/"><i className="bi bi-box text-5xl"></i></Link>

                <Show when={!isAuthenticated && !isLoginPage}>
                    <Link to="/auth/login" className="flex items-center justify-center text-lg w-36 h-12 px-4 rounded-md bg-primary text-white font-semibold">
                        Login
                    </Link>
                </Show>

            </Show>

        </header>
    )
}
