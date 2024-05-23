
import { Show } from "../components/ui/Show"
import { Header } from "../components/Header"
import { Navbar } from "../components/Navbar"
import { useAuth } from "../store/AuthContext"
import { PropsWithChildren } from "react"

export const MainLayout = ({ children }: PropsWithChildren) => {

    const { isAuthenticated } = useAuth()

    return (

        <div className="h-screen w-screen flex flex-row">

            <Show when={isAuthenticated}><Navbar /></Show>

            <main className="h-full w-full flex flex-col px-6 md:px-12 lg:px-28 py-8">

                <Header />

                <div className="w-full h-full">

                    {children}

                </div>

            </main>

        </div>
    )
}