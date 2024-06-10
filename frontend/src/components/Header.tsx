
import { Show } from "./ui/Show"
import { useAuth } from "../store/AuthContext"
import { useAppStore } from "../store/AppStore"
import { useModalStore } from "../store/ModalStore"
import { toUpperAndLower } from "../lib/string"
import { Link, useLocation } from "react-router-dom"

export const Header = () => {

    const { pageTitle } = useAppStore()
    const { user, role } = useAuth()
    const { setModal } = useModalStore()
    const { isAuthenticated } = useAuth()

    const location = useLocation()
    const isLoginPage = location.pathname === "/auth/login"
    const isRootPage = location.pathname === "/"

    const handleManageAccount = async () => setModal("manageAccount")

    return (

        <header className="w-full h-16 flex flex-row items-center justify-between text-neutral-950 pt-10">

            <Show when={isAuthenticated}>

                <Show when={isRootPage}>
                    <div className="flex flex-row gap-8 items-center">
                        <Link to="/"><i className="bi bi-box text-5xl"></i></Link>
                        <h1 className="text-2xl font-bold">{`Welcome again ${user?.name}!`}</h1>
                    </div>
                </Show>

                <Show when={!isRootPage}>
                    <h1 className="text-3xl font-bold">{pageTitle}</h1>
                </Show>

                <div className="flex flex-row gap-12">

                    <Show when={isRootPage}>
                        <Link to="/dashboard" className="flex items-center justify-center text-lg w-36 h-12 px-4 rounded-md border-2 border-primary text-primary font-semibold">
                            Dashboard
                        </Link>
                    </Show>

                    <Show when={!isRootPage}>

                        <div className="flex flex-row items-center gap-8">
                            <div className="flex flex-col items-end">
                                <h4 className="font-semibold">{user?.name}</h4>
                                <h5>{toUpperAndLower(role?.toString() || "")}</h5>
                            </div>

                            <div onClick={() => handleManageAccount()} className="bg-primary p-1 rounded-full">
                                <i className="text-white text-4xl bi bi-person-circle"></i>
                            </div>
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
