
import { Show } from "../components/ui/Show"
import { Header } from "../components/Header"
import { Navbar } from "../components/Navbar"
import { useAuth } from "../store/AuthContext"
import { useLocation } from "react-router-dom"
import { PropsWithChildren } from "react"

export const MainLayout = ({ children }: PropsWithChildren) => {

    const location = useLocation()
    const isRootPage = location.pathname === "/"

    const { isAuthenticated } = useAuth()

    return (

        <div className="h-screen w-screen flex flex-row">

            <Show when={isAuthenticated && !isRootPage}><Navbar /></Show>

            <main className="h-full w-full flex flex-col px-12 lg:px-10 xl:16 2xl:px-28 py-8">

                <Header />

                <div className="w-full h-full">

                    {children}

                </div>

            </main>

        </div>
    )
}